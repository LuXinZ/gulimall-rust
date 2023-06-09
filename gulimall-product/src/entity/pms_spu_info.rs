//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "pms_spu_info")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub spu_name: Option<String>,
    pub spu_description: Option<String>,
    pub catalog_id: Option<i64>,
    pub brand_id: Option<i64>,
    #[sea_orm(column_type = "Decimal(Some((18, 4)))", nullable)]
    pub weight: Option<Decimal>,
    pub publish_status: Option<i8>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
