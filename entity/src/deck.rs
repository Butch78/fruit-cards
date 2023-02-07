use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "decks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: String,
    pub color: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    // TODO Add user_id
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
