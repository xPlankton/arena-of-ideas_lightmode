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
pub struct OpenLootboxArgs {
    pub id: u64,
}

impl Reducer for OpenLootboxArgs {
    const REDUCER_NAME: &'static str = "open_lootbox";
}

#[allow(unused)]
pub fn open_lootbox(id: u64) {
    OpenLootboxArgs { id }.invoke();
}

#[allow(unused)]
pub fn on_open_lootbox(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &u64) + Send + 'static,
) -> ReducerCallbackId<OpenLootboxArgs> {
    OpenLootboxArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let OpenLootboxArgs { id } = __args;
        __callback(__identity, __addr, __status, id);
    })
}

#[allow(unused)]
pub fn once_on_open_lootbox(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &u64) + Send + 'static,
) -> ReducerCallbackId<OpenLootboxArgs> {
    OpenLootboxArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let OpenLootboxArgs { id } = __args;
        __callback(__identity, __addr, __status, id);
    })
}

#[allow(unused)]
pub fn remove_on_open_lootbox(id: ReducerCallbackId<OpenLootboxArgs>) {
    OpenLootboxArgs::remove_on_reducer(id);
}
