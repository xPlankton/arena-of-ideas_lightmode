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
    Address,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct GlobalSettings {
    pub always_zero: u32,
    pub shop_slots_min: u32,
    pub shop_slots_max: u32,
    pub shop_slots_per_round: f32,
    pub shop_g_start: i32,
    pub shop_g_income_min: i32,
    pub shop_g_income_max: i32,
    pub shop_g_income_per_round: i32,
    pub shop_price_reroll: i32,
    pub shop_price_unit: i32,
    pub team_slots: u32,
}

impl TableType for GlobalSettings {
    const TABLE_NAME: &'static str = "GlobalSettings";
    type ReducerEvent = super::ReducerEvent;
}

impl GlobalSettings {
    #[allow(unused)]
    pub fn filter_by_always_zero(always_zero: u32) -> Option<Self> {
        Self::find(|row| row.always_zero == always_zero)
    }
    #[allow(unused)]
    pub fn filter_by_shop_slots_min(shop_slots_min: u32) -> TableIter<Self> {
        Self::filter(|row| row.shop_slots_min == shop_slots_min)
    }
    #[allow(unused)]
    pub fn filter_by_shop_slots_max(shop_slots_max: u32) -> TableIter<Self> {
        Self::filter(|row| row.shop_slots_max == shop_slots_max)
    }
    #[allow(unused)]
    pub fn filter_by_shop_slots_per_round(shop_slots_per_round: f32) -> TableIter<Self> {
        Self::filter(|row| row.shop_slots_per_round == shop_slots_per_round)
    }
    #[allow(unused)]
    pub fn filter_by_shop_g_start(shop_g_start: i32) -> TableIter<Self> {
        Self::filter(|row| row.shop_g_start == shop_g_start)
    }
    #[allow(unused)]
    pub fn filter_by_shop_g_income_min(shop_g_income_min: i32) -> TableIter<Self> {
        Self::filter(|row| row.shop_g_income_min == shop_g_income_min)
    }
    #[allow(unused)]
    pub fn filter_by_shop_g_income_max(shop_g_income_max: i32) -> TableIter<Self> {
        Self::filter(|row| row.shop_g_income_max == shop_g_income_max)
    }
    #[allow(unused)]
    pub fn filter_by_shop_g_income_per_round(shop_g_income_per_round: i32) -> TableIter<Self> {
        Self::filter(|row| row.shop_g_income_per_round == shop_g_income_per_round)
    }
    #[allow(unused)]
    pub fn filter_by_shop_price_reroll(shop_price_reroll: i32) -> TableIter<Self> {
        Self::filter(|row| row.shop_price_reroll == shop_price_reroll)
    }
    #[allow(unused)]
    pub fn filter_by_shop_price_unit(shop_price_unit: i32) -> TableIter<Self> {
        Self::filter(|row| row.shop_price_unit == shop_price_unit)
    }
    #[allow(unused)]
    pub fn filter_by_team_slots(team_slots: u32) -> TableIter<Self> {
        Self::filter(|row| row.team_slots == team_slots)
    }
}
