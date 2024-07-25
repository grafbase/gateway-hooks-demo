use core::fmt;
use std::sync::LazyLock;

use futures_util::TryFutureExt;
use reqwest::Client;
use tracing::Level;

use crate::bindings::component::grafbase::types::{Context, Headers};

/// This URL is to the authorized-subgraph server, which must be running before we start
/// calling the gateway.
static URL: &str = "http://localhost:4000/hello";

static REQWEST: LazyLock<Client> = LazyLock::new(Client::new);

/// The JSON response from the /hello endpoint.
#[derive(serde::Deserialize, Debug)]
struct Response {
    hello: String,
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Hello, {}", self.hello))
    }
}

pub(super) async fn call(context: Context, headers: Headers) -> anyhow::Result<()> {
    let user_id = headers.get("x-current-user-id");

    // Spans can store information that is added to every log call we trigger so
    // long as we have the _guard variable in the scope.
    let span = tracing::span!(Level::INFO, "gateway request", user_id = user_id);
    let _guard = span.enter();

    if let Some(id) = headers.get("x-current-user-id") {
        context.set("current-user-id", &id);
    }

    // Three HTTP requests executed asynchronously in Tokio. The time this takes
    // is the longest response time of any of these request futures.
    let (first, second, third) = tokio::try_join!(request(), request(), request(),)?;

    // Output the response data to the stdout.
    tracing::info!("response from first http call: {first}");
    tracing::info!("response from second http call: {second}");
    tracing::info!("response from third http call: {third}");

    Ok(())
}

async fn request() -> anyhow::Result<Response> {
    let response = REQWEST
        .get(URL)
        .send()
        .and_then(|response| response.json())
        .await?;

    Ok(response)
}
