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
pub struct NewTeamArgs {
    pub name: String,
}

impl Reducer for NewTeamArgs {
    const REDUCER_NAME: &'static str = "new_team";
}

#[allow(unused)]
pub fn new_team(name: String) {
    NewTeamArgs { name }.invoke();
}

#[allow(unused)]
pub fn on_new_team(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &String) + Send + 'static,
) -> ReducerCallbackId<NewTeamArgs> {
    NewTeamArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let NewTeamArgs { name } = __args;
        __callback(__identity, __addr, __status, name);
    })
}

#[allow(unused)]
pub fn once_on_new_team(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &String) + Send + 'static,
) -> ReducerCallbackId<NewTeamArgs> {
    NewTeamArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let NewTeamArgs { name } = __args;
        __callback(__identity, __addr, __status, name);
    })
}

#[allow(unused)]
pub fn remove_on_new_team(id: ReducerCallbackId<NewTeamArgs>) {
    NewTeamArgs::remove_on_reducer(id);
}
