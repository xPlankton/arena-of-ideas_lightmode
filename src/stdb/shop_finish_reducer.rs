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
pub struct ShopFinishArgs {}

impl Reducer for ShopFinishArgs {
    const REDUCER_NAME: &'static str = "shop_finish";
}

#[allow(unused)]
pub fn shop_finish() {
    ShopFinishArgs {}.invoke();
}

#[allow(unused)]
pub fn on_shop_finish(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status) + Send + 'static,
) -> ReducerCallbackId<ShopFinishArgs> {
    ShopFinishArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let ShopFinishArgs {} = __args;
        __callback(__identity, __addr, __status);
    })
}

#[allow(unused)]
pub fn once_on_shop_finish(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status) + Send + 'static,
) -> ReducerCallbackId<ShopFinishArgs> {
    ShopFinishArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let ShopFinishArgs {} = __args;
        __callback(__identity, __addr, __status);
    })
}

#[allow(unused)]
pub fn remove_on_shop_finish(id: ReducerCallbackId<ShopFinishArgs>) {
    ShopFinishArgs::remove_on_reducer(id);
}
