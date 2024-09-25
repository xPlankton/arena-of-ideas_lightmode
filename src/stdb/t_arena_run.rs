// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused_imports)]
use super::fusion::Fusion;
use super::game_mode::GameMode;
use super::reward::Reward;
use super::shop_slot::ShopSlot;
use super::team_slot::TeamSlot;
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
pub struct TArenaRun {
    pub mode: GameMode,
    pub id: u64,
    pub owner: u64,
    pub team: u64,
    pub battles: Vec<u64>,
    pub enemies: Vec<u64>,
    pub shop_slots: Vec<ShopSlot>,
    pub team_slots: Vec<TeamSlot>,
    pub fusion: Option<Fusion>,
    pub g: i32,
    pub price_reroll: i32,
    pub free_rerolls: u32,
    pub lives: u32,
    pub active: bool,
    pub finale: bool,
    pub floor: u32,
    pub rerolls: u32,
    pub rewards: Vec<Reward>,
    pub streak: u32,
    pub last_updated: u64,
}

impl TableType for TArenaRun {
    const TABLE_NAME: &'static str = "TArenaRun";
    type ReducerEvent = super::ReducerEvent;
}

impl TableWithPrimaryKey for TArenaRun {
    type PrimaryKey = u64;
    fn primary_key(&self) -> &Self::PrimaryKey {
        &self.id
    }
}

impl TArenaRun {
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
    pub fn find_by_owner(owner: u64) -> Option<Self> {
        Self::find(|row| row.owner == owner)
    }
    #[allow(unused)]
    pub fn filter_by_team(team: u64) -> TableIter<Self> {
        Self::filter(|row| row.team == team)
    }
    #[allow(unused)]
    pub fn filter_by_g(g: i32) -> TableIter<Self> {
        Self::filter(|row| row.g == g)
    }
    #[allow(unused)]
    pub fn filter_by_price_reroll(price_reroll: i32) -> TableIter<Self> {
        Self::filter(|row| row.price_reroll == price_reroll)
    }
    #[allow(unused)]
    pub fn filter_by_free_rerolls(free_rerolls: u32) -> TableIter<Self> {
        Self::filter(|row| row.free_rerolls == free_rerolls)
    }
    #[allow(unused)]
    pub fn filter_by_lives(lives: u32) -> TableIter<Self> {
        Self::filter(|row| row.lives == lives)
    }
    #[allow(unused)]
    pub fn filter_by_active(active: bool) -> TableIter<Self> {
        Self::filter(|row| row.active == active)
    }
    #[allow(unused)]
    pub fn filter_by_finale(finale: bool) -> TableIter<Self> {
        Self::filter(|row| row.finale == finale)
    }
    #[allow(unused)]
    pub fn filter_by_floor(floor: u32) -> TableIter<Self> {
        Self::filter(|row| row.floor == floor)
    }
    #[allow(unused)]
    pub fn filter_by_rerolls(rerolls: u32) -> TableIter<Self> {
        Self::filter(|row| row.rerolls == rerolls)
    }
    #[allow(unused)]
    pub fn filter_by_streak(streak: u32) -> TableIter<Self> {
        Self::filter(|row| row.streak == streak)
    }
    #[allow(unused)]
    pub fn filter_by_last_updated(last_updated: u64) -> TableIter<Self> {
        Self::filter(|row| row.last_updated == last_updated)
    }
}
