// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

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
pub struct SetPasswordArgs {
    pub old_pass: String,
    pub new_pass: String,
}

impl Reducer for SetPasswordArgs {
    const REDUCER_NAME: &'static str = "set_password";
}

#[allow(unused)]
pub fn set_password(old_pass: String, new_pass: String) {
    SetPasswordArgs { old_pass, new_pass }.invoke();
}

#[allow(unused)]
pub fn on_set_password(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &String, &String) + Send + 'static,
) -> ReducerCallbackId<SetPasswordArgs> {
    SetPasswordArgs::on_reducer(move |__identity, __addr, __status, __args| {
        let SetPasswordArgs { old_pass, new_pass } = __args;
        __callback(__identity, __addr, __status, old_pass, new_pass);
    })
}

#[allow(unused)]
pub fn once_on_set_password(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &String, &String) + Send + 'static,
) -> ReducerCallbackId<SetPasswordArgs> {
    SetPasswordArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
        let SetPasswordArgs { old_pass, new_pass } = __args;
        __callback(__identity, __addr, __status, old_pass, new_pass);
    })
}

#[allow(unused)]
pub fn remove_on_set_password(id: ReducerCallbackId<SetPasswordArgs>) {
    SetPasswordArgs::remove_on_reducer(id);
}
