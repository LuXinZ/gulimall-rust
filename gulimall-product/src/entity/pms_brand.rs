//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "pms_brand")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub brand_id: i64,
    pub name: Option<String>,
    pub logo: Option<String>,
    #[sea_orm(column_type = "custom(\"LONGTEXT\")", nullable)]
    pub descript: Option<String>,
    pub show_status: Option<i8>,
    pub first_letter: Option<String>,
    pub sort: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
