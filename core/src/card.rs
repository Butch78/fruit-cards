use ::entity::{card, card::Entity as Card};
use sea_orm::*;

// import chrono to get current time
use chrono::Utc;

pub struct CardQuery;

impl CardQuery {
    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Option<card::Model>, DbErr> {
        Card::find_by_id(id).one(db).await
    }

    pub async fn find_in_page(
        db: &DbConn,
        page: u64,
        cards_per_page: u64,
    ) -> Result<(Vec<card::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Card::find()
            .order_by_asc(card::Column::Id)
            .paginate(db, cards_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated cards
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn create(db: &DbConn, form_data: card::Model) -> Result<card::ActiveModel, DbErr> {
        let now = Utc::now().naive_utc().to_string();

        card::ActiveModel {
            created_at: Set(now.to_owned()),
            question: Set(form_data.question.to_owned()),
            answer: Set(form_data.answer.to_owned()),
            state: Set(form_data.state.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_by_id(
        db: &DbConn,
        id: i32,
        form_data: card::Model,
    ) -> Result<card::Model, DbErr> {
        let card: card::ActiveModel = Card::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find card.".to_owned()))
            .map(Into::into)?;

        card::ActiveModel {
            id: card.id,
            question: Set(form_data.question.to_owned()),
            answer: Set(form_data.answer.to_owned()),
            state: Set(form_data.state.to_owned()),
            created_at: Set(form_data.created_at.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn delete(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let card: card::ActiveModel = Card::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find card.".to_owned()))
            .map(Into::into)?;

        card.delete(db).await
    }
}
