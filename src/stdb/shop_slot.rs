// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused_imports)]
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ShopSlot {
    pub unit: String,
    pub id: u64,
    pub buy_price: i32,
    pub stack_price: i32,
    pub freeze: bool,
    pub discount: bool,
    pub available: bool,
    pub stack_targets: Vec<u8>,
    pub house_filter: Vec<String>,
}
