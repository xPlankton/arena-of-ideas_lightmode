use std::str::FromStr;

use super::*;

#[spacetimedb(table)]
pub struct UserAccess {
    #[primarykey]
    identity: Identity,
    rights: Vec<UserRight>,
}

#[derive(SpacetimeType, Debug, Eq, PartialEq)]
pub enum UserRight {
    UnitSync,
}

const SERVER_IDENTITY_HEX: &'static str =
    "93dda09db9a56d8fa6c024d843e805d8262191db3b4ba84c5efcd1ad451fed4e";

#[spacetimedb(reducer)]
fn give_right(ctx: ReducerContext, identity: String) -> Result<(), String> {
    if !hex::encode(&ctx.sender.as_bytes()).eq(SERVER_IDENTITY_HEX) {
        return Err("Sender identity doesn't match server".to_owned());
    }
    let identity = Identity::from_str(&identity).map_err(|e| e.to_string())?;
    let right = UserRight::UnitSync;
    if let Some(mut access) = UserAccess::filter_by_identity(&identity) {
        if !access.rights.contains(&right) {
            access.rights.push(right);
            UserAccess::update_by_identity(&identity, access);
        }
    } else {
        UserAccess::insert(UserAccess {
            identity,
            rights: [right].into(),
        })?;
    };
    Ok(())
}

impl UserRight {
    pub fn check(self, identity: &Identity) -> Result<(), String> {
        if UserAccess::filter_by_identity(identity).is_some_and(|v| v.rights.contains(&self)) {
            Ok(())
        } else {
            Err(format!("No right {self:?}"))
        }
    }
}
