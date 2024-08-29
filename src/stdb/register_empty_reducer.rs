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
pub struct RegisterEmptyArgs {}

impl Reducer for RegisterEmptyArgs {
    const REDUCER_NAME: &'static str = "register_empty";
}

#[allow(unused)]
pub fn register_empty() {
    RegisterEmptyArgs {}.invoke();
}

#[allow(unused)]
pub fn on_register_empty(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status) + Send + 'static,
) -> ReducerCallbackId<RegisterEmptyArgs> {
    RegisterEmptyArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let RegisterEmptyArgs {} = __args;
        __callback(__identity, __addr, __status);
    })
}

#[allow(unused)]
pub fn once_on_register_empty(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status) + Send + 'static,
) -> ReducerCallbackId<RegisterEmptyArgs> {
    RegisterEmptyArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let RegisterEmptyArgs {} = __args;
        __callback(__identity, __addr, __status);
    })
}

#[allow(unused)]
pub fn remove_on_register_empty(id: ReducerCallbackId<RegisterEmptyArgs>) {
    RegisterEmptyArgs::remove_on_reducer(id);
}
