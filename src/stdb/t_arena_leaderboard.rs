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
pub struct TArenaLeaderboard {
    pub mode: GameMode,
    pub season: u32,
    pub round: u32,
    pub score: u32,
    pub user: u64,
    pub team: u64,
    pub run: u64,
    pub ts: u64,
}

impl TableType for TArenaLeaderboard {
    const TABLE_NAME: &'static str = "TArenaLeaderboard";
    type ReducerEvent = super::ReducerEvent;
}

impl TArenaLeaderboard {
    #[allow(unused)]
    pub fn filter_by_season(season: u32) -> TableIter<Self> {
        Self::filter(|row| row.season == season)
    }
    #[allow(unused)]
    pub fn filter_by_round(round: u32) -> TableIter<Self> {
        Self::filter(|row| row.round == round)
    }
    #[allow(unused)]
    pub fn filter_by_score(score: u32) -> TableIter<Self> {
        Self::filter(|row| row.score == score)
    }
    #[allow(unused)]
    pub fn filter_by_user(user: u64) -> TableIter<Self> {
        Self::filter(|row| row.user == user)
    }
    #[allow(unused)]
    pub fn filter_by_team(team: u64) -> TableIter<Self> {
        Self::filter(|row| row.team == team)
    }
    #[allow(unused)]
    pub fn filter_by_run(run: u64) -> TableIter<Self> {
        Self::filter(|row| row.run == run)
    }
    #[allow(unused)]
    pub fn filter_by_ts(ts: u64) -> TableIter<Self> {
        Self::filter(|row| row.ts == ts)
    }
}
