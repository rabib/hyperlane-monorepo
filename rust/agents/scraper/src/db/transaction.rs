//! SeaORM Entity. Generated by sea-orm-codegen 0.10.0

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "transaction"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq)]
pub struct Model {
    pub id: i64,
    pub time_created: TimeDateTime,
    pub hash: String,
    pub block_id: i64,
    pub gas_used: Decimal,
    pub sender: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    TimeCreated,
    Hash,
    BlockId,
    GasUsed,
    Sender,
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
    Block,
    CheckpointUpdate,
    Message,
    GasPayment,
    DeliveredMessage,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::BigInteger.def(),
            Self::TimeCreated => ColumnType::DateTime.def(),
            Self::Hash => ColumnType::String(Some(64u32)).def().unique(),
            Self::BlockId => ColumnType::BigInteger.def(),
            Self::GasUsed => ColumnType::Decimal(Some((78u32, 18u32))).def(),
            Self::Sender => ColumnType::String(Some(64u32)).def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Block => Entity::belongs_to(super::block::Entity)
                .from(Column::BlockId)
                .to(super::block::Column::Id)
                .into(),
            Self::CheckpointUpdate => Entity::has_many(super::checkpoint_update::Entity).into(),
            Self::Message => Entity::has_many(super::message::Entity).into(),
            Self::GasPayment => Entity::has_many(super::gas_payment::Entity).into(),
            Self::DeliveredMessage => Entity::has_many(super::delivered_message::Entity).into(),
        }
    }
}

impl Related<super::block::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Block.def()
    }
}

impl Related<super::checkpoint_update::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CheckpointUpdate.def()
    }
}

impl Related<super::message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Message.def()
    }
}

impl Related<super::gas_payment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GasPayment.def()
    }
}

impl Related<super::delivered_message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DeliveredMessage.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
