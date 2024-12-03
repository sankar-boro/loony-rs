use axum::{
    extract::DefaultBodyLimit,
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use tower::ServiceBuilder;
use tower_http::limit::RequestBodyLimitLayer;

use crate::search::{insert, search};
use crate::AppState;
use serde_json::json;
use tower_http::cors::CorsLayer;

pub async fn home() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        Json(json!({"sankar": "boro"})),
    ))
}

pub async fn create_router(connection: AppState, cors: CorsLayer) -> Router {
    let search_routes = Router::new()
        .route("/insert", post(insert))
        .route("/query/:query", get(search));

    Router::new()
        .nest("/search", search_routes)
        .route("/", get(home))
        .with_state(connection)
        .layer(cors)
        .layer(DefaultBodyLimit::disable())
        .layer(
            ServiceBuilder::new()
                .layer(RequestBodyLimitLayer::new(12 * 1024 * 1024))
                .into_inner(),
        )
}
