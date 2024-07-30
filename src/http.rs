use crate::google_ads;
use axum::{http::StatusCode, routing::Router, Json};
use dotenvy_macro::dotenv;
use tokio::net::TcpListener;
use tokio::signal;

// #[derive(Clone)]
// pub struct AppState {
//     pub db: PgPool,
// }

fn api_router() -> Router {
    let api_routes = Router::new().merge(google_ads::router::router());
    Router::new().nest("/v1", api_routes).fallback(api_fallback)
}

async fn api_fallback() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "message": "Not Found" })),
    )
}

pub async fn serve() {
    let app = api_router().into_make_service();
    let addr = format!(
        "0.0.0.0:{}",
        dotenv!("PORT")
            .parse::<u32>()
            .expect("can get port from env")
    );
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("can start server");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
