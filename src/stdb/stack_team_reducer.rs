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
pub struct StackTeamArgs {
    pub source: u8,
    pub target: u8,
}

impl Reducer for StackTeamArgs {
    const REDUCER_NAME: &'static str = "stack_team";
}

#[allow(unused)]
pub fn stack_team(source: u8, target: u8) {
    StackTeamArgs { source, target }.invoke();
}

#[allow(unused)]
pub fn on_stack_team(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &u8, &u8) + Send + 'static,
) -> ReducerCallbackId<StackTeamArgs> {
    StackTeamArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let StackTeamArgs { source, target } = __args;
        __callback(__identity, __addr, __status, source, target);
    })
}

#[allow(unused)]
pub fn once_on_stack_team(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &u8, &u8) + Send + 'static,
) -> ReducerCallbackId<StackTeamArgs> {
    StackTeamArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let StackTeamArgs { source, target } = __args;
        __callback(__identity, __addr, __status, source, target);
    })
}

#[allow(unused)]
pub fn remove_on_stack_team(id: ReducerCallbackId<StackTeamArgs>) {
    StackTeamArgs::remove_on_reducer(id);
}
