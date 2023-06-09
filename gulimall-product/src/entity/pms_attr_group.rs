//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "pms_attr_group")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub attr_group_id: i64,
    pub attr_group_name: Option<String>,
    pub sort: Option<i32>,
    pub descript: Option<String>,
    pub icon: Option<String>,
    pub catelog_id: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
