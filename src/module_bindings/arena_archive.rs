// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

use super::table_unit::TableUnit;
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
pub struct ArenaArchive {
    pub id: u64,
    pub user_id: u64,
    pub round: u32,
    pub wins: u32,
    pub loses: u32,
    pub team: Vec<TableUnit>,
    pub timestamp: u64,
}

impl TableType for ArenaArchive {
    const TABLE_NAME: &'static str = "ArenaArchive";
    type ReducerEvent = super::ReducerEvent;
}

impl TableWithPrimaryKey for ArenaArchive {
    type PrimaryKey = u64;
    fn primary_key(&self) -> &Self::PrimaryKey {
        &self.id
    }
}

impl ArenaArchive {
    #[allow(unused)]
    pub fn filter_by_id(id: u64) -> Option<Self> {
        Self::find(|row| row.id == id)
    }
    #[allow(unused)]
    pub fn filter_by_user_id(user_id: u64) -> TableIter<Self> {
        Self::filter(|row| row.user_id == user_id)
    }
    #[allow(unused)]
    pub fn filter_by_round(round: u32) -> TableIter<Self> {
        Self::filter(|row| row.round == round)
    }
    #[allow(unused)]
    pub fn filter_by_wins(wins: u32) -> TableIter<Self> {
        Self::filter(|row| row.wins == wins)
    }
    #[allow(unused)]
    pub fn filter_by_loses(loses: u32) -> TableIter<Self> {
        Self::filter(|row| row.loses == loses)
    }
    #[allow(unused)]
    pub fn filter_by_team(team: Vec<TableUnit>) -> TableIter<Self> {
        Self::filter(|row| row.team == team)
    }
    #[allow(unused)]
    pub fn filter_by_timestamp(timestamp: u64) -> TableIter<Self> {
        Self::filter(|row| row.timestamp == timestamp)
    }
}
