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
pub struct DailyUpdateTimer {
    pub scheduled_id: u64,
    pub scheduled_at: ScheduleAt,
}

impl TableType for DailyUpdateTimer {
    const TABLE_NAME: &'static str = "DailyUpdateTimer";
    type ReducerEvent = super::ReducerEvent;
}

impl TableWithPrimaryKey for DailyUpdateTimer {
    type PrimaryKey = u64;
    fn primary_key(&self) -> &Self::PrimaryKey {
        &self.scheduled_id
    }
}

impl DailyUpdateTimer {
    #[allow(unused)]
    pub fn filter_by_scheduled_id(scheduled_id: u64) -> TableIter<Self> {
        Self::filter(|row| row.scheduled_id == scheduled_id)
    }
    #[allow(unused)]
    pub fn find_by_scheduled_id(scheduled_id: u64) -> Option<Self> {
        Self::find(|row| row.scheduled_id == scheduled_id)
    }
}
