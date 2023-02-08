use axum::{
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::flash::{get_flash_cookie, post_response, FlashData, PostResponse};
use crate::utilities::{app_status::AppState, params::Params};

use tower_cookies::Cookies;

use serde::{Deserialize, Serialize};

use chrono::Utc;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card {
    pub id: i32,
    pub created_at: String,
    pub question: String,
    pub answer: String,
    pub state: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateDeck {
    pub user_id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Deck {
    pub id: i32,
    pub user_id: i32,
    pub created_at: String,
    pub name: String,
    pub description: String,
    pub state: String,
    pub cards: Vec<Card>,
}

pub fn created_at() -> String {
    Utc::now().to_string()
}

pub async fn edit_deck(
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    println!("deck: {:?}", id);
    Ok(Json(Deck {
        id: 1,
        user_id: 1,
        created_at: created_at(),
        name: "Software Systems".to_string(),
        description: "A Deck for Software System".to_string(),
        state: "Fresh".to_string(),
        cards: vec![],
    }))
}

pub async fn create_deck(
    Json(input): Json<CreateDeck>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    println!("deck: {:?}", input);
    let deck = Deck {
        id: 1,
        user_id: input.user_id,
        created_at: created_at(),
        name: input.name,
        description: input.description,
        state: "test".to_string(),
        cards: vec![],
    };

    Ok(Json(deck))
}

pub async fn list_decks(
    state: State<AppState>,
    Query(params): Query<Params>,
    cookies: Cookies,
) -> Result<Json<Vec<Deck>>, (StatusCode, &'static str)> {
    let card_1 = Card {
        id: 1,
        created_at: created_at(),
        question: "What is a Google Server?".to_string(),
        answer: "A Google Server is a commodity-class x86 PC server with custom Linux, running outdated hardware and with its own 12V battery to counter unstable power supplies. Each server rack holds 40 to 80 servers and is setup in standard shipping containers and wired together.".to_string(),
        state: "Fresh".to_string(),
    };

    let card_2 = Card {
        id: 2,
        created_at: created_at(),
        question: "What is the purpose of a Google Server?".to_string(),
        answer: "The purpose of a Google Server is to provide a distributed database system for a globe-spanning web application.".to_string(),
        state: "Rotten".to_string(),
    };

    let cards = vec![card_1, card_2];

    let deck = Deck {
        id: 1,
        user_id: 1,
        created_at: created_at(),
        name: "Software Systems".to_string(),
        description: "test".to_string(),
        state: "test".to_string(),
        cards: cards.clone(),
    };

    let deck_2 = Deck {
        id: 2,
        user_id: 1,
        created_at: created_at(),
        name: "FSS01_02_TowardsScalableNoSQL".to_string(),
        description: "test".to_string(),
        state: "test".to_string(),
        cards: cards,
    };

    let decks = vec![deck, deck_2];

    Ok(Json(decks))
}
