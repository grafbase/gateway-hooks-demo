use axum::{response::IntoResponse, routing::post, Json, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/authorize-user", post(authorize_user))
        .route("/authorize-address", post(authorize_address));

    println!("Serving authorization service at 127.0.0.1:4001");
    axum::serve(TcpListener::bind("127.0.0.1:4001").await?, app).await?;

    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct AuthorizationResponse {
    authorized: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct AuthorizeUserRequest {
    current_user_id: usize,
    user_id: usize,
}

async fn authorize_user(Json(payload): Json<AuthorizeUserRequest>) -> impl IntoResponse {
    let is_authorized = payload.user_id <= payload.current_user_id;
    println!(
        "Authorizing access to user {} for user {}: {is_authorized}",
        payload.user_id, payload.current_user_id
    );
    Json(AuthorizationResponse {
        authorized: is_authorized,
    })
}

#[derive(serde::Serialize, serde::Deserialize)]
struct AuthorizeAddressRequest {
    current_user_id: usize,
    owner_id: usize,
}

async fn authorize_address(Json(payload): Json<AuthorizeAddressRequest>) -> impl IntoResponse {
    let is_authorized = payload.owner_id == payload.current_user_id;
    println!(
        "Authorizing access to address of user {} for user {}: {is_authorized}",
        payload.owner_id, payload.current_user_id
    );
    Json(AuthorizationResponse {
        authorized: is_authorized,
    })
}
