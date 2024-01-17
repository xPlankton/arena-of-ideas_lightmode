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
pub struct RunSubmitResultArgs {
    pub win: bool,
}

impl Reducer for RunSubmitResultArgs {
    const REDUCER_NAME: &'static str = "run_submit_result";
}

#[allow(unused)]
pub fn run_submit_result(win: bool) {
    RunSubmitResultArgs { win }.invoke();
}

#[allow(unused)]
pub fn on_run_submit_result(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &bool) + Send + 'static,
) -> ReducerCallbackId<RunSubmitResultArgs> {
    RunSubmitResultArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let RunSubmitResultArgs { win } = __args;
        __callback(__identity, __addr, __status, win);
    })
}

#[allow(unused)]
pub fn once_on_run_submit_result(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &bool) + Send + 'static,
) -> ReducerCallbackId<RunSubmitResultArgs> {
    RunSubmitResultArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let RunSubmitResultArgs { win } = __args;
        __callback(__identity, __addr, __status, win);
    })
}

#[allow(unused)]
pub fn remove_on_run_submit_result(id: ReducerCallbackId<RunSubmitResultArgs>) {
    RunSubmitResultArgs::remove_on_reducer(id);
}
