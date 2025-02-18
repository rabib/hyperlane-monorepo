//! SeaORM Entity. Generated by sea-orm-codegen 0.10.0

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "delivered_message"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq)]
pub struct Model {
    pub id: i64,
    pub time_created: TimeDateTime,
    pub msg_id: Option<i64>,
    pub hash: String,
    pub domain: i32,
    pub inbox_address: String,
    pub tx_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    TimeCreated,
    MsgId,
    Hash,
    Domain,
    InboxAddress,
    TxId,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i64;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Domain,
    Message,
    Transaction,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::BigInteger.def(),
            Self::TimeCreated => ColumnType::DateTime.def(),
            Self::MsgId => ColumnType::BigInteger.def().null().unique(),
            Self::Hash => ColumnType::String(Some(64u32)).def().unique(),
            Self::Domain => ColumnType::Integer.def(),
            Self::InboxAddress => ColumnType::String(Some(64u32)).def(),
            Self::TxId => ColumnType::BigInteger.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Domain => Entity::belongs_to(super::domain::Entity)
                .from(Column::Domain)
                .to(super::domain::Column::Id)
                .into(),
            Self::Message => Entity::belongs_to(super::message::Entity)
                .from(Column::MsgId)
                .to(super::message::Column::Id)
                .into(),
            Self::Transaction => Entity::belongs_to(super::transaction::Entity)
                .from(Column::TxId)
                .to(super::transaction::Column::Id)
                .into(),
        }
    }
}

impl Related<super::domain::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Domain.def()
    }
}

impl Related<super::message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Message.def()
    }
}

impl Related<super::transaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Transaction.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
