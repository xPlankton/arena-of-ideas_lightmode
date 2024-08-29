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
    Address, ScheduleAt,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TArenaRunArchive {
    pub mode: GameMode,
    pub id: u64,
    pub owner: u64,
    pub team: u64,
    pub battles: Vec<u64>,
    pub round: u32,
}

impl TableType for TArenaRunArchive {
    const TABLE_NAME: &'static str = "TArenaRunArchive";
    type ReducerEvent = super::ReducerEvent;
}

impl TableWithPrimaryKey for TArenaRunArchive {
    type PrimaryKey = u64;
    fn primary_key(&self) -> &Self::PrimaryKey {
        &self.id
    }
}

impl TArenaRunArchive {
    #[allow(unused)]
    pub fn filter_by_id(id: u64) -> TableIter<Self> {
        Self::filter(|row| row.id == id)
    }
    #[allow(unused)]
    pub fn find_by_id(id: u64) -> Option<Self> {
        Self::find(|row| row.id == id)
    }
    #[allow(unused)]
    pub fn filter_by_owner(owner: u64) -> TableIter<Self> {
        Self::filter(|row| row.owner == owner)
    }
    #[allow(unused)]
    pub fn filter_by_team(team: u64) -> TableIter<Self> {
        Self::filter(|row| row.team == team)
    }
    #[allow(unused)]
    pub fn filter_by_round(round: u32) -> TableIter<Self> {
        Self::filter(|row| row.round == round)
    }
}
