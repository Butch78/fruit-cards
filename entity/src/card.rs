use sea_orm::entity::prelude::*;

// #[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
// #[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
// pub enum Status {
//     #[sea_orm(string_value = "A")]
//     Active,
//     #[sea_orm(string_value = "I")]
//     Inactive,
// }

// #[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
// #[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
// pub enum Difficulty {
//     #[sea_orm(string_value = "E")]
//     Easy,
//     #[sea_orm(string_value = "M")]
//     Medium,
//     #[sea_orm(string_value = "H")]
//     Hard,
// }

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "cards")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub question: String,
    #[sea_orm(column_type = "Text")]
    pub answer: String,
    //pub status: Status,
    //pub status_option: Option<Status>,
    pub color: String,
    pub difficulty: Difficulty,
    // pub difficulty_option: Option<Difficulty>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
