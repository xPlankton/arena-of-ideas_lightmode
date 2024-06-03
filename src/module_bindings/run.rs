// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused_imports)]
use super::fusion::Fusion;
use super::shop_slot::ShopSlot;
use super::team_slot::TeamSlot;
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
pub struct Run {
    pub id: u64,
    pub user_id: u64,
    pub team: Vec<TeamSlot>,
    pub shop: Vec<ShopSlot>,
    pub fusion: Option<Fusion>,
    pub round: u32,
    pub last_updated: u64,
}

impl TableType for Run {
    const TABLE_NAME: &'static str = "Run";
    type ReducerEvent = super::ReducerEvent;
}

impl TableWithPrimaryKey for Run {
    type PrimaryKey = u64;
    fn primary_key(&self) -> &Self::PrimaryKey {
        &self.id
    }
}

impl Run {
    #[allow(unused)]
    pub fn filter_by_id(id: u64) -> Option<Self> {
        Self::find(|row| row.id == id)
    }
    #[allow(unused)]
    pub fn filter_by_user_id(user_id: u64) -> Option<Self> {
        Self::find(|row| row.user_id == user_id)
    }
    #[allow(unused)]
    pub fn filter_by_team(team: Vec<TeamSlot>) -> TableIter<Self> {
        Self::filter(|row| row.team == team)
    }
    #[allow(unused)]
    pub fn filter_by_shop(shop: Vec<ShopSlot>) -> TableIter<Self> {
        Self::filter(|row| row.shop == shop)
    }
    #[allow(unused)]
    pub fn filter_by_fusion(fusion: Option<Fusion>) -> TableIter<Self> {
        Self::filter(|row| row.fusion == fusion)
    }
    #[allow(unused)]
    pub fn filter_by_round(round: u32) -> TableIter<Self> {
        Self::filter(|row| row.round == round)
    }
    #[allow(unused)]
    pub fn filter_by_last_updated(last_updated: u64) -> TableIter<Self> {
        Self::filter(|row| row.last_updated == last_updated)
    }
}
