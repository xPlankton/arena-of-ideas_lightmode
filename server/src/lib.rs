mod ability;
mod arena;
mod arena_leaderboard;
mod arena_pool;
mod base_unit;
mod battle;
mod fused_unit;
mod global_data;
mod global_settings;
mod house;
mod representation;
mod status;
mod sync;
mod team;
mod user;

use anyhow::Context;
pub use arena_leaderboard::*;
pub use arena_pool::*;
pub use battle::*;
pub use fused_unit::*;
pub use global_data::*;
pub use global_settings::*;
use rand::{thread_rng, RngCore};
pub use spacetimedb::SpacetimeType;
pub use spacetimedb::{spacetimedb, Identity, ReducerContext};
pub use team::*;
pub use user::*;

pub type GID = u64;

trait StrContext<T> {
    fn context_str(self, str: &'static str) -> Result<T, String>;
    fn with_context_str<F>(self, f: F) -> Result<T, String>
    where
        F: FnOnce() -> String;
}

impl<T> StrContext<T> for Option<T> {
    fn context_str(self, str: &'static str) -> Result<T, String> {
        self.context(str).map_err(|e| e.to_string())
    }

    fn with_context_str<F>(self, f: F) -> Result<T, String>
    where
        F: FnOnce() -> String,
    {
        self.with_context(f).map_err(|e| e.to_string())
    }
}

#[spacetimedb(init)]
fn init() -> Result<(), String> {
    GlobalData::init()?;
    TArenaLeaderboard::insert(TArenaLeaderboard::new(
        thread_rng().next_u32(),
        thread_rng().next_u32(),
        1,
        1,
        1,
    ));
    TArenaLeaderboard::insert(TArenaLeaderboard::new(
        thread_rng().next_u32(),
        thread_rng().next_u32(),
        1,
        1,
        1,
    ));
    TArenaLeaderboard::insert(TArenaLeaderboard::new(
        thread_rng().next_u32(),
        thread_rng().next_u32(),
        1,
        1,
        1,
    ));
    TArenaLeaderboard::insert(TArenaLeaderboard::new(
        thread_rng().next_u32(),
        thread_rng().next_u32(),
        1,
        1,
        1,
    ));
    TArenaLeaderboard::insert(TArenaLeaderboard::new(
        thread_rng().next_u32(),
        thread_rng().next_u32(),
        1,
        1,
        1,
    ));
    TArenaLeaderboard::insert(TArenaLeaderboard::new(
        thread_rng().next_u32(),
        thread_rng().next_u32(),
        1,
        1,
        1,
    ));
    Ok(())
}

pub fn next_id() -> GID {
    GlobalData::next_id()
}
