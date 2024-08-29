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
pub struct RegisterArgs {
    pub name: String,
    pub pass: String,
}

impl Reducer for RegisterArgs {
    const REDUCER_NAME: &'static str = "register";
}

#[allow(unused)]
pub fn register(name: String, pass: String) {
    RegisterArgs { name, pass }.invoke();
}

#[allow(unused)]
pub fn on_register(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &String, &String) + Send + 'static,
) -> ReducerCallbackId<RegisterArgs> {
    RegisterArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let RegisterArgs { name, pass } = __args;
        __callback(__identity, __addr, __status, name, pass);
    })
}

#[allow(unused)]
pub fn once_on_register(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &String, &String) + Send + 'static,
) -> ReducerCallbackId<RegisterArgs> {
    RegisterArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let RegisterArgs { name, pass } = __args;
        __callback(__identity, __addr, __status, name, pass);
    })
}

#[allow(unused)]
pub fn remove_on_register(id: ReducerCallbackId<RegisterArgs>) {
    RegisterArgs::remove_on_reducer(id);
}
