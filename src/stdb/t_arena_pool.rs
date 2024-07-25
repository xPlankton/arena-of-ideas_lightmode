// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused_imports)]
use super::game_mode::GameMode;
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
pub struct TArenaPool {
    pub mode: GameMode,
    pub team: u64,
    pub round: u32,
}

impl TableType for TArenaPool {
    const TABLE_NAME: &'static str = "TArenaPool";
    type ReducerEvent = super::ReducerEvent;
}

impl TableWithPrimaryKey for TArenaPool {
    type PrimaryKey = u64;
    fn primary_key(&self) -> &Self::PrimaryKey {
        &self.team
    }
}

impl TArenaPool {
    #[allow(unused)]
    pub fn filter_by_mode(mode: GameMode) -> TableIter<Self> {
        Self::filter(|row| row.mode == mode)
    }
    #[allow(unused)]
    pub fn filter_by_team(team: u64) -> Option<Self> {
        Self::find(|row| row.team == team)
    }
    #[allow(unused)]
    pub fn filter_by_round(round: u32) -> TableIter<Self> {
        Self::filter(|row| row.round == round)
    }
}
