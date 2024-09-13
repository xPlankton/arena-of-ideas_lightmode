// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused_imports)]
use super::item_kind::ItemKind;
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address, ScheduleAt,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TMetaShop {
    pub id: u64,
    pub item_kind: ItemKind,
    pub price: i64,
}

impl TableType for TMetaShop {
    const TABLE_NAME: &'static str = "TMetaShop";
    type ReducerEvent = super::ReducerEvent;
}

impl TableWithPrimaryKey for TMetaShop {
    type PrimaryKey = u64;
    fn primary_key(&self) -> &Self::PrimaryKey {
        &self.id
    }
}

impl TMetaShop {
    #[allow(unused)]
    pub fn filter_by_id(id: u64) -> TableIter<Self> {
        Self::filter(|row| row.id == id)
    }
    #[allow(unused)]
    pub fn find_by_id(id: u64) -> Option<Self> {
        Self::find(|row| row.id == id)
    }
    #[allow(unused)]
    pub fn filter_by_price(price: i64) -> TableIter<Self> {
        Self::filter(|row| row.price == price)
    }
}
