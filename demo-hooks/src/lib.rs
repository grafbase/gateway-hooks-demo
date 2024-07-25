use std::sync::{LazyLock, Once};

use bindings::{
    component::grafbase::types::{Context, Error, Headers},
    exports::component::grafbase::gateway_request,
};
use tokio::runtime::Runtime;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[allow(warnings)]
mod bindings;

mod request;

struct Component;

/// We initialize this once for the whole component lifetime.
/// It is a single-threaded Tokio runtime, which can execute async rust code.
static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
});

impl gateway_request::Guest for Component {
    fn on_gateway_request(context: Context, headers: Headers) -> Result<(), Error> {
        init_logging();

        match RUNTIME.block_on(request::call(context, headers)) {
            Ok(()) => Ok(()),
            Err(e) => Err(Error {
                extensions: Vec::new(),
                message: format!("internal error: {e}"),
            }),
        }
    }
}

/// Initializes the log subscriber, which must be called in the beginning of every hook to get output.
/// When the hook is called once, the Once construct prevents re-initializing the logger, which is already
/// in the component memory.
fn init_logging() {
    static LOG: Once = Once::new();

    LOG.call_once(|| {
        let log_layer = tracing_subscriber::fmt::layer()
            .with_ansi(true)
            .with_target(true);

        tracing_subscriber::registry()
            .with(log_layer)
            .with(EnvFilter::new("info"))
            .init();
    });
}

bindings::export!(Component with_types_in bindings);
