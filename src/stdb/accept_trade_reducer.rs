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
    Address, ScheduleAt,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AcceptTradeArgs {
    pub id: u64,
}

impl Reducer for AcceptTradeArgs {
    const REDUCER_NAME: &'static str = "accept_trade";
}

#[allow(unused)]
pub fn accept_trade(id: u64) {
    AcceptTradeArgs { id }.invoke();
}

#[allow(unused)]
pub fn on_accept_trade(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &u64) + Send + 'static,
) -> ReducerCallbackId<AcceptTradeArgs> {
    AcceptTradeArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let AcceptTradeArgs { id } = __args;
        __callback(__identity, __addr, __status, id);
    })
}

#[allow(unused)]
pub fn once_on_accept_trade(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &u64) + Send + 'static,
) -> ReducerCallbackId<AcceptTradeArgs> {
    AcceptTradeArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let AcceptTradeArgs { id } = __args;
        __callback(__identity, __addr, __status, id);
    })
}

#[allow(unused)]
pub fn remove_on_accept_trade(id: ReducerCallbackId<AcceptTradeArgs>) {
    AcceptTradeArgs::remove_on_reducer(id);
}
