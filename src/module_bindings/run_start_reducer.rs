// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#[allow(unused)]
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
pub struct RunStartArgs {}

impl Reducer for RunStartArgs {
    const REDUCER_NAME: &'static str = "run_start";
}

#[allow(unused)]
pub fn run_start() {
    RunStartArgs {}.invoke();
}

#[allow(unused)]
pub fn on_run_start(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status) + Send + 'static,
) -> ReducerCallbackId<RunStartArgs> {
    RunStartArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let RunStartArgs {} = __args;
        __callback(__identity, __addr, __status);
    })
}

#[allow(unused)]
pub fn once_on_run_start(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status) + Send + 'static,
) -> ReducerCallbackId<RunStartArgs> {
    RunStartArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let RunStartArgs {} = __args;
        __callback(__identity, __addr, __status);
    })
}

#[allow(unused)]
pub fn remove_on_run_start(id: ReducerCallbackId<RunStartArgs>) {
    RunStartArgs::remove_on_reducer(id);
}
