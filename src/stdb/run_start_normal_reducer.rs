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
pub struct RunStartNormalArgs {}

impl Reducer for RunStartNormalArgs {
    const REDUCER_NAME: &'static str = "run_start_normal";
}

#[allow(unused)]
pub fn run_start_normal() {
    RunStartNormalArgs {}.invoke();
}

#[allow(unused)]
pub fn on_run_start_normal(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status) + Send + 'static,
) -> ReducerCallbackId<RunStartNormalArgs> {
    RunStartNormalArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let RunStartNormalArgs {} = __args;
        __callback(__identity, __addr, __status);
    })
}

#[allow(unused)]
pub fn once_on_run_start_normal(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status) + Send + 'static,
) -> ReducerCallbackId<RunStartNormalArgs> {
    RunStartNormalArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let RunStartNormalArgs {} = __args;
        __callback(__identity, __addr, __status);
    })
}

#[allow(unused)]
pub fn remove_on_run_start_normal(id: ReducerCallbackId<RunStartNormalArgs>) {
    RunStartNormalArgs::remove_on_reducer(id);
}
