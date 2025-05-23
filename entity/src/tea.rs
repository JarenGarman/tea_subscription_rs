//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "tea")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub description: String,
    pub brew_time: Time,
    pub image_url: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::subscription_tea::Entity")]
    SubscriptionTea,
}

impl Related<super::subscription_tea::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SubscriptionTea.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
