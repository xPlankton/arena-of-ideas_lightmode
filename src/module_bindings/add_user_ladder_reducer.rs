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
pub struct AddUserLadderArgs {
    pub levels: Vec<String>,
}

impl Reducer for AddUserLadderArgs {
    const REDUCER_NAME: &'static str = "add_user_ladder";
}

#[allow(unused)]
pub fn add_user_ladder(levels: Vec<String>) {
    AddUserLadderArgs { levels }.invoke();
}

#[allow(unused)]
pub fn on_add_user_ladder(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &Vec<String>) + Send + 'static,
) -> ReducerCallbackId<AddUserLadderArgs> {
    AddUserLadderArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let AddUserLadderArgs { levels } = __args;
        __callback(__identity, __addr, __status, levels);
    })
}

#[allow(unused)]
pub fn once_on_add_user_ladder(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &Vec<String>) + Send + 'static,
) -> ReducerCallbackId<AddUserLadderArgs> {
    AddUserLadderArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let AddUserLadderArgs { levels } = __args;
        __callback(__identity, __addr, __status, levels);
    })
}

#[allow(unused)]
pub fn remove_on_add_user_ladder(id: ReducerCallbackId<AddUserLadderArgs>) {
    AddUserLadderArgs::remove_on_reducer(id);
}
