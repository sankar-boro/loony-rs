mod error;
mod route;
mod search;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use search::{init_search, Search};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
#[allow(dead_code)]
pub struct Dirs {
    tmp_upload: String,
    blog_upload: String,
    book_upload: String,
    user_upload: String,
}

#[derive(Clone)]
pub struct AppState {
    pub search: Search,
}

async fn init() -> AppState {
    return AppState {
        search: init_search(),
    };
}

#[tokio::main]
async fn main() {
    // log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    let host = std::env::var("HOST").unwrap();
    let port = std::env::var("PORT").unwrap();
    let origins = std::env::var("ORIGINS").unwrap();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let connection = init().await;

    // Parse the comma-separated string into a Vec<String>
    let origins: Vec<HeaderValue> = origins
        .split(',')
        .map(|s| s.parse::<HeaderValue>().unwrap())
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let router = route::create_router(connection, cors).await;
    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}
