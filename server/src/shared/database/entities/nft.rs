//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "nft")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub token_id: String,
    pub token_address: String,
    pub name: String,
    pub image_url: String,
    pub original_url: String,
    pub end_date: DateTime,
    pub block_number: i32,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub is_active: bool,
    #[sea_orm(column_type = "Double")]
    pub square_price: f64,
    pub position: i32,
    pub last_position: i32,
    pub position_within_block: i32,
    pub last_crawl_date: DateTime,
    pub collection_id: i32,
    pub user_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::collection::Entity",
        from = "Column::CollectionId",
        to = "super::collection::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Collection,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    User,
}

impl Related<super::collection::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Collection.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
