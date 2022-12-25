use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum Status {
    #[sea_orm(rename = "A")]
    Active,
    #[sea_orm(rename = "I")]
    Inactive,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum difficulty {
    #[sea_orm(rename = "E")]
    Easy,
    #[sea_orm(rename = "M")]
    Medium,
    #[sea_orm(rename = "H")]
    Hard,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "cards")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub text: String,
    pub status: Status,
    pub status_option: Option<Status>,
    pub color: String,
    pub difficulty: difficulty,
    pub difficulty_option: Option<difficulty>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
