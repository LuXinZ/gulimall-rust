//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "pms_sku_images")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub sku_id: Option<i64>,
    pub img_url: Option<String>,
    pub img_sort: Option<i32>,
    pub default_img: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
