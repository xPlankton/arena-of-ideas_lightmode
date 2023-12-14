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
pub struct FinishBuildingLadderArgs {}

impl Reducer for FinishBuildingLadderArgs {
    const REDUCER_NAME: &'static str = "finish_building_ladder";
}

#[allow(unused)]
pub fn finish_building_ladder() {
    FinishBuildingLadderArgs {}.invoke();
}

#[allow(unused)]
pub fn on_finish_building_ladder(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status) + Send + 'static,
) -> ReducerCallbackId<FinishBuildingLadderArgs> {
    FinishBuildingLadderArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let FinishBuildingLadderArgs {} = __args;
        __callback(__identity, __addr, __status);
    })
}

#[allow(unused)]
pub fn once_on_finish_building_ladder(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status) + Send + 'static,
) -> ReducerCallbackId<FinishBuildingLadderArgs> {
    FinishBuildingLadderArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let FinishBuildingLadderArgs {} = __args;
        __callback(__identity, __addr, __status);
    })
}

#[allow(unused)]
pub fn remove_on_finish_building_ladder(id: ReducerCallbackId<FinishBuildingLadderArgs>) {
    FinishBuildingLadderArgs::remove_on_reducer(id);
}
