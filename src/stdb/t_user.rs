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
pub struct TUser {
    pub id: u64,
    pub name: String,
    pub identities: Vec<Identity>,
    pub pass_hash: Option<String>,
    pub online: bool,
    pub last_login: u64,
}

impl TableType for TUser {
    const TABLE_NAME: &'static str = "TUser";
    type ReducerEvent = super::ReducerEvent;
}

impl TableWithPrimaryKey for TUser {
    type PrimaryKey = u64;
    fn primary_key(&self) -> &Self::PrimaryKey {
        &self.id
    }
}

impl TUser {
    #[allow(unused)]
    pub fn filter_by_id(id: u64) -> TableIter<Self> {
        Self::filter(|row| row.id == id)
    }
    #[allow(unused)]
    pub fn find_by_id(id: u64) -> Option<Self> {
        Self::find(|row| row.id == id)
    }
    #[allow(unused)]
    pub fn filter_by_name(name: String) -> TableIter<Self> {
        Self::filter(|row| row.name == name)
    }
    #[allow(unused)]
    pub fn find_by_name(name: String) -> Option<Self> {
        Self::find(|row| row.name == name)
    }
    #[allow(unused)]
    pub fn filter_by_online(online: bool) -> TableIter<Self> {
        Self::filter(|row| row.online == online)
    }
    #[allow(unused)]
    pub fn filter_by_last_login(last_login: u64) -> TableIter<Self> {
        Self::filter(|row| row.last_login == last_login)
    }
}
