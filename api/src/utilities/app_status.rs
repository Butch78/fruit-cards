use axum::extract::FromRef;
use fruit_cards_core::sea_orm::DatabaseConnection;

use tera::Tera;

// TODO add JWT

#[derive(Clone, FromRef)]
pub struct AppState {
    pub templates: Tera,
    pub conn: DatabaseConnection,
}
