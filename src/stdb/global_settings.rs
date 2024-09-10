// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused_imports)]
use super::arena_settings::ArenaSettings;
use super::battle_settings::BattleSettings;
use super::meta_settings::MetaSettings;
use super::rarity_settings::RaritySettings;
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
pub struct GlobalSettings {
    pub always_zero: u32,
    pub arena: ArenaSettings,
    pub rarities: RaritySettings,
    pub battle: BattleSettings,
    pub craft_shards_cost: u32,
    pub meta: MetaSettings,
    pub ghost_unit: String,
}

impl TableType for GlobalSettings {
    const TABLE_NAME: &'static str = "GlobalSettings";
    type ReducerEvent = super::ReducerEvent;
}

impl GlobalSettings {
    #[allow(unused)]
    pub fn filter_by_always_zero(always_zero: u32) -> TableIter<Self> {
        Self::filter(|row| row.always_zero == always_zero)
    }
    #[allow(unused)]
    pub fn find_by_always_zero(always_zero: u32) -> Option<Self> {
        Self::find(|row| row.always_zero == always_zero)
    }
    #[allow(unused)]
    pub fn filter_by_craft_shards_cost(craft_shards_cost: u32) -> TableIter<Self> {
        Self::filter(|row| row.craft_shards_cost == craft_shards_cost)
    }
    #[allow(unused)]
    pub fn filter_by_ghost_unit(ghost_unit: String) -> TableIter<Self> {
        Self::filter(|row| row.ghost_unit == ghost_unit)
    }
}
