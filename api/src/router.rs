use crate::{
    routes::{
        card::list_cards,
        deck::{create_deck, list_decks, edit_deck},
        health_check::health_check,
        post::{create, delete_post, edit_post, list_posts, new_post, update_post},
    },
    utilities::app_status::AppState,
};

use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};

use tower_http::services::ServeDir;

use axum::{
    http::Method,
    http::StatusCode,
    routing::{get, get_service, post},
    Router,
};

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health_check", get(health_check))
        .route("/", get(list_posts).post(create))
        .route("/:id", get(edit_post).post(update_post))
        .route("/new", get(new_post))
        .route("/delete/:id", post(delete_post))
        .route("/api/cards", get(list_cards))
        .route("/api/decks", get(list_decks).post(create_deck))
        .route("/api/decks/:id", get(edit_deck))
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
        .layer(cors)
}
