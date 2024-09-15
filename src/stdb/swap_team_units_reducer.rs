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
pub struct SwapTeamUnitsArgs {
    pub team: u64,
    pub from: u8,
    pub to: u8,
}

impl Reducer for SwapTeamUnitsArgs {
    const REDUCER_NAME: &'static str = "swap_team_units";
}

#[allow(unused)]
pub fn swap_team_units(team: u64, from: u8, to: u8) {
    SwapTeamUnitsArgs { team, from, to }.invoke();
}

#[allow(unused)]
pub fn on_swap_team_units(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &u64, &u8, &u8) + Send + 'static,
) -> ReducerCallbackId<SwapTeamUnitsArgs> {
    SwapTeamUnitsArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let SwapTeamUnitsArgs { team, from, to } = __args;
        __callback(__identity, __addr, __status, team, from, to);
    })
}

#[allow(unused)]
pub fn once_on_swap_team_units(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &u64, &u8, &u8) + Send + 'static,
) -> ReducerCallbackId<SwapTeamUnitsArgs> {
    SwapTeamUnitsArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let SwapTeamUnitsArgs { team, from, to } = __args;
        __callback(__identity, __addr, __status, team, from, to);
    })
}

#[allow(unused)]
pub fn remove_on_swap_team_units(id: ReducerCallbackId<SwapTeamUnitsArgs>) {
    SwapTeamUnitsArgs::remove_on_reducer(id);
}
