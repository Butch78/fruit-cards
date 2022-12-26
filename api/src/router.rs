use crate::{
    routes::{
        health_check::health_check,
        post::{create_post, delete_post, edit_post, list_posts, new_post, update_post},
    },
    utilities::app_status::AppState,
};

use tower_cookies::CookieManagerLayer;

use tower_http::services::ServeDir;

use axum::{
    http::StatusCode,
    routing::{get, get_service, post},
    Router,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/", get(list_posts).post(create_post))
        .route("/:id", get(edit_post).post(update_post))
        .route("/new", get(new_post))
        .route("/delete/:id", post(delete_post))
        .nest_service(
            "/static",
            get_service(ServeDir::new(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/static"
            )))
            .handle_error(|error: std::io::Error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {error}"),
                )
            }),
        )
        .layer(CookieManagerLayer::new())
        .with_state(state)
}
