use axum::{
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::Json,
};



use crate::flash::{get_flash_cookie, post_response, FlashData, PostResponse};
use crate::utilities::{app_status::AppState, params::Params};

use entity::card;
use fruit_cards_core::CardQuery;
use tower_cookies::Cookies;

pub async fn list_cards(
    state: State<AppState>,
    Query(params): Query<Params>,
    cookies: Cookies,
) -> Result<Json<Vec<card::Model>>, (StatusCode, &'static str)> {
    
    let card = card::Model {
        id: 1,
        created_at: "test".to_string(),
        question: "test".to_string(),
        answer: "test".to_string(),
        state: "test".to_string(),
    };

    let cards = vec![card];

    Ok(Json(cards))
}