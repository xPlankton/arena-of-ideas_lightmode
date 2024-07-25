// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused_imports)]
use spacetimedb_sdk::callbacks::{DbCallbacks, ReducerCallbacks};
use spacetimedb_sdk::client_api_messages::{Event, TableUpdate};
use spacetimedb_sdk::client_cache::{ClientCache, RowCallbackReminders};
use spacetimedb_sdk::global_connection::with_connection_mut;
use spacetimedb_sdk::identity::Credentials;
use spacetimedb_sdk::reducer::AnyReducerEvent;
use spacetimedb_sdk::spacetime_module::SpacetimeModule;
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address,
};
use std::sync::Arc;

pub mod arena_settings;
pub mod fuse_cancel_reducer;
pub mod fuse_choose_reducer;
pub mod fuse_start_reducer;
pub mod fused_unit;
pub mod fusion;
pub mod game_mode;
pub mod global_data;
pub mod global_settings;
pub mod login_by_identity_reducer;
pub mod login_reducer;
pub mod logout_reducer;
pub mod rarity_settings;
pub mod register_empty_reducer;
pub mod register_reducer;
pub mod run_finish_reducer;
pub mod run_start_daily_reducer;
pub mod run_start_normal_reducer;
pub mod set_name_reducer;
pub mod set_password_reducer;
pub mod shop_buy_reducer;
pub mod shop_change_g_reducer;
pub mod shop_finish_reducer;
pub mod shop_reorder_reducer;
pub mod shop_reroll_reducer;
pub mod shop_sell_reducer;
pub mod shop_slot;
pub mod stack_shop_reducer;
pub mod stack_team_reducer;
pub mod submit_battle_result_reducer;
pub mod t_ability;
pub mod t_arena_leaderboard;
pub mod t_arena_pool;
pub mod t_arena_run;
pub mod t_arena_run_archive;
pub mod t_base_unit;
pub mod t_battle;
pub mod t_battle_result;
pub mod t_house;
pub mod t_representation;
pub mod t_status;
pub mod t_team;
pub mod t_user;
pub mod team_slot;
pub mod upload_assets_reducer;
pub mod upload_game_archive_reducer;

pub use arena_settings::*;
pub use fuse_cancel_reducer::*;
pub use fuse_choose_reducer::*;
pub use fuse_start_reducer::*;
pub use fused_unit::*;
pub use fusion::*;
pub use game_mode::*;
pub use global_data::*;
pub use global_settings::*;
pub use login_by_identity_reducer::*;
pub use login_reducer::*;
pub use logout_reducer::*;
pub use rarity_settings::*;
pub use register_empty_reducer::*;
pub use register_reducer::*;
pub use run_finish_reducer::*;
pub use run_start_daily_reducer::*;
pub use run_start_normal_reducer::*;
pub use set_name_reducer::*;
pub use set_password_reducer::*;
pub use shop_buy_reducer::*;
pub use shop_change_g_reducer::*;
pub use shop_finish_reducer::*;
pub use shop_reorder_reducer::*;
pub use shop_reroll_reducer::*;
pub use shop_sell_reducer::*;
pub use shop_slot::*;
pub use stack_shop_reducer::*;
pub use stack_team_reducer::*;
pub use submit_battle_result_reducer::*;
pub use t_ability::*;
pub use t_arena_leaderboard::*;
pub use t_arena_pool::*;
pub use t_arena_run::*;
pub use t_arena_run_archive::*;
pub use t_base_unit::*;
pub use t_battle::*;
pub use t_battle_result::*;
pub use t_house::*;
pub use t_representation::*;
pub use t_status::*;
pub use t_team::*;
pub use t_user::*;
pub use team_slot::*;
pub use upload_assets_reducer::*;
pub use upload_game_archive_reducer::*;

#[allow(unused)]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum ReducerEvent {
    FuseCancel(fuse_cancel_reducer::FuseCancelArgs),
    FuseChoose(fuse_choose_reducer::FuseChooseArgs),
    FuseStart(fuse_start_reducer::FuseStartArgs),
    Login(login_reducer::LoginArgs),
    LoginByIdentity(login_by_identity_reducer::LoginByIdentityArgs),
    Logout(logout_reducer::LogoutArgs),
    Register(register_reducer::RegisterArgs),
    RegisterEmpty(register_empty_reducer::RegisterEmptyArgs),
    RunFinish(run_finish_reducer::RunFinishArgs),
    RunStartDaily(run_start_daily_reducer::RunStartDailyArgs),
    RunStartNormal(run_start_normal_reducer::RunStartNormalArgs),
    SetName(set_name_reducer::SetNameArgs),
    SetPassword(set_password_reducer::SetPasswordArgs),
    ShopBuy(shop_buy_reducer::ShopBuyArgs),
    ShopChangeG(shop_change_g_reducer::ShopChangeGArgs),
    ShopFinish(shop_finish_reducer::ShopFinishArgs),
    ShopReorder(shop_reorder_reducer::ShopReorderArgs),
    ShopReroll(shop_reroll_reducer::ShopRerollArgs),
    ShopSell(shop_sell_reducer::ShopSellArgs),
    StackShop(stack_shop_reducer::StackShopArgs),
    StackTeam(stack_team_reducer::StackTeamArgs),
    SubmitBattleResult(submit_battle_result_reducer::SubmitBattleResultArgs),
    UploadAssets(upload_assets_reducer::UploadAssetsArgs),
    UploadGameArchive(upload_game_archive_reducer::UploadGameArchiveArgs),
}

#[allow(unused)]
pub struct Module;
impl SpacetimeModule for Module {
    fn handle_table_update(
        &self,
        table_update: TableUpdate,
        client_cache: &mut ClientCache,
        callbacks: &mut RowCallbackReminders,
    ) {
        let table_name = &table_update.table_name[..];
        match table_name {
            "GlobalData" => client_cache
                .handle_table_update_no_primary_key::<global_data::GlobalData>(
                    callbacks,
                    table_update,
                ),
            "GlobalSettings" => client_cache
                .handle_table_update_no_primary_key::<global_settings::GlobalSettings>(
                    callbacks,
                    table_update,
                ),
            "TAbility" => client_cache.handle_table_update_with_primary_key::<t_ability::TAbility>(
                callbacks,
                table_update,
            ),
            "TArenaLeaderboard" => client_cache
                .handle_table_update_no_primary_key::<t_arena_leaderboard::TArenaLeaderboard>(
                    callbacks,
                    table_update,
                ),
            "TArenaPool" => client_cache
                .handle_table_update_with_primary_key::<t_arena_pool::TArenaPool>(
                    callbacks,
                    table_update,
                ),
            "TArenaRun" => client_cache
                .handle_table_update_with_primary_key::<t_arena_run::TArenaRun>(
                    callbacks,
                    table_update,
                ),
            "TArenaRunArchive" => client_cache
                .handle_table_update_with_primary_key::<t_arena_run_archive::TArenaRunArchive>(
                    callbacks,
                    table_update,
                ),
            "TBaseUnit" => client_cache
                .handle_table_update_with_primary_key::<t_base_unit::TBaseUnit>(
                    callbacks,
                    table_update,
                ),
            "TBattle" => client_cache
                .handle_table_update_with_primary_key::<t_battle::TBattle>(callbacks, table_update),
            "THouse" => client_cache
                .handle_table_update_with_primary_key::<t_house::THouse>(callbacks, table_update),
            "TRepresentation" => client_cache
                .handle_table_update_no_primary_key::<t_representation::TRepresentation>(
                    callbacks,
                    table_update,
                ),
            "TStatus" => client_cache
                .handle_table_update_with_primary_key::<t_status::TStatus>(callbacks, table_update),
            "TTeam" => client_cache
                .handle_table_update_with_primary_key::<t_team::TTeam>(callbacks, table_update),
            "TUser" => client_cache
                .handle_table_update_with_primary_key::<t_user::TUser>(callbacks, table_update),
            _ => {
                spacetimedb_sdk::log::error!("TableRowOperation on unknown table {:?}", table_name)
            }
        }
    }
    fn invoke_row_callbacks(
        &self,
        reminders: &mut RowCallbackReminders,
        worker: &mut DbCallbacks,
        reducer_event: Option<Arc<AnyReducerEvent>>,
        state: &Arc<ClientCache>,
    ) {
        reminders.invoke_callbacks::<global_data::GlobalData>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<global_settings::GlobalSettings>(
            worker,
            &reducer_event,
            state,
        );
        reminders.invoke_callbacks::<t_ability::TAbility>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<t_arena_leaderboard::TArenaLeaderboard>(
            worker,
            &reducer_event,
            state,
        );
        reminders.invoke_callbacks::<t_arena_pool::TArenaPool>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<t_arena_run::TArenaRun>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<t_arena_run_archive::TArenaRunArchive>(
            worker,
            &reducer_event,
            state,
        );
        reminders.invoke_callbacks::<t_base_unit::TBaseUnit>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<t_battle::TBattle>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<t_house::THouse>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<t_representation::TRepresentation>(
            worker,
            &reducer_event,
            state,
        );
        reminders.invoke_callbacks::<t_status::TStatus>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<t_team::TTeam>(worker, &reducer_event, state);
        reminders.invoke_callbacks::<t_user::TUser>(worker, &reducer_event, state);
    }
    fn handle_event(
        &self,
        event: Event,
        _reducer_callbacks: &mut ReducerCallbacks,
        _state: Arc<ClientCache>,
    ) -> Option<Arc<AnyReducerEvent>> {
        let Some(function_call) = &event.function_call else {
            spacetimedb_sdk::log::warn!("Received Event with None function_call");
            return None;
        };
        #[allow(clippy::match_single_binding)]
match &function_call.reducer[..] {
						"fuse_cancel" => _reducer_callbacks.handle_event_of_type::<fuse_cancel_reducer::FuseCancelArgs, ReducerEvent>(event, _state, ReducerEvent::FuseCancel),
			"fuse_choose" => _reducer_callbacks.handle_event_of_type::<fuse_choose_reducer::FuseChooseArgs, ReducerEvent>(event, _state, ReducerEvent::FuseChoose),
			"fuse_start" => _reducer_callbacks.handle_event_of_type::<fuse_start_reducer::FuseStartArgs, ReducerEvent>(event, _state, ReducerEvent::FuseStart),
			"login" => _reducer_callbacks.handle_event_of_type::<login_reducer::LoginArgs, ReducerEvent>(event, _state, ReducerEvent::Login),
			"login_by_identity" => _reducer_callbacks.handle_event_of_type::<login_by_identity_reducer::LoginByIdentityArgs, ReducerEvent>(event, _state, ReducerEvent::LoginByIdentity),
			"logout" => _reducer_callbacks.handle_event_of_type::<logout_reducer::LogoutArgs, ReducerEvent>(event, _state, ReducerEvent::Logout),
			"register" => _reducer_callbacks.handle_event_of_type::<register_reducer::RegisterArgs, ReducerEvent>(event, _state, ReducerEvent::Register),
			"register_empty" => _reducer_callbacks.handle_event_of_type::<register_empty_reducer::RegisterEmptyArgs, ReducerEvent>(event, _state, ReducerEvent::RegisterEmpty),
			"run_finish" => _reducer_callbacks.handle_event_of_type::<run_finish_reducer::RunFinishArgs, ReducerEvent>(event, _state, ReducerEvent::RunFinish),
			"run_start_daily" => _reducer_callbacks.handle_event_of_type::<run_start_daily_reducer::RunStartDailyArgs, ReducerEvent>(event, _state, ReducerEvent::RunStartDaily),
			"run_start_normal" => _reducer_callbacks.handle_event_of_type::<run_start_normal_reducer::RunStartNormalArgs, ReducerEvent>(event, _state, ReducerEvent::RunStartNormal),
			"set_name" => _reducer_callbacks.handle_event_of_type::<set_name_reducer::SetNameArgs, ReducerEvent>(event, _state, ReducerEvent::SetName),
			"set_password" => _reducer_callbacks.handle_event_of_type::<set_password_reducer::SetPasswordArgs, ReducerEvent>(event, _state, ReducerEvent::SetPassword),
			"shop_buy" => _reducer_callbacks.handle_event_of_type::<shop_buy_reducer::ShopBuyArgs, ReducerEvent>(event, _state, ReducerEvent::ShopBuy),
			"shop_change_g" => _reducer_callbacks.handle_event_of_type::<shop_change_g_reducer::ShopChangeGArgs, ReducerEvent>(event, _state, ReducerEvent::ShopChangeG),
			"shop_finish" => _reducer_callbacks.handle_event_of_type::<shop_finish_reducer::ShopFinishArgs, ReducerEvent>(event, _state, ReducerEvent::ShopFinish),
			"shop_reorder" => _reducer_callbacks.handle_event_of_type::<shop_reorder_reducer::ShopReorderArgs, ReducerEvent>(event, _state, ReducerEvent::ShopReorder),
			"shop_reroll" => _reducer_callbacks.handle_event_of_type::<shop_reroll_reducer::ShopRerollArgs, ReducerEvent>(event, _state, ReducerEvent::ShopReroll),
			"shop_sell" => _reducer_callbacks.handle_event_of_type::<shop_sell_reducer::ShopSellArgs, ReducerEvent>(event, _state, ReducerEvent::ShopSell),
			"stack_shop" => _reducer_callbacks.handle_event_of_type::<stack_shop_reducer::StackShopArgs, ReducerEvent>(event, _state, ReducerEvent::StackShop),
			"stack_team" => _reducer_callbacks.handle_event_of_type::<stack_team_reducer::StackTeamArgs, ReducerEvent>(event, _state, ReducerEvent::StackTeam),
			"submit_battle_result" => _reducer_callbacks.handle_event_of_type::<submit_battle_result_reducer::SubmitBattleResultArgs, ReducerEvent>(event, _state, ReducerEvent::SubmitBattleResult),
			"upload_assets" => _reducer_callbacks.handle_event_of_type::<upload_assets_reducer::UploadAssetsArgs, ReducerEvent>(event, _state, ReducerEvent::UploadAssets),
			"upload_game_archive" => _reducer_callbacks.handle_event_of_type::<upload_game_archive_reducer::UploadGameArchiveArgs, ReducerEvent>(event, _state, ReducerEvent::UploadGameArchive),
			unknown => { spacetimedb_sdk::log::error!("Event on an unknown reducer: {:?}", unknown); None }
}
    }
    fn handle_resubscribe(
        &self,
        new_subs: TableUpdate,
        client_cache: &mut ClientCache,
        callbacks: &mut RowCallbackReminders,
    ) {
        let table_name = &new_subs.table_name[..];
        match table_name {
            "GlobalData" => client_cache
                .handle_resubscribe_for_type::<global_data::GlobalData>(callbacks, new_subs),
            "GlobalSettings" => client_cache
                .handle_resubscribe_for_type::<global_settings::GlobalSettings>(
                    callbacks, new_subs,
                ),
            "TAbility" => {
                client_cache.handle_resubscribe_for_type::<t_ability::TAbility>(callbacks, new_subs)
            }
            "TArenaLeaderboard" => client_cache
                .handle_resubscribe_for_type::<t_arena_leaderboard::TArenaLeaderboard>(
                    callbacks, new_subs,
                ),
            "TArenaPool" => client_cache
                .handle_resubscribe_for_type::<t_arena_pool::TArenaPool>(callbacks, new_subs),
            "TArenaRun" => client_cache
                .handle_resubscribe_for_type::<t_arena_run::TArenaRun>(callbacks, new_subs),
            "TArenaRunArchive" => client_cache
                .handle_resubscribe_for_type::<t_arena_run_archive::TArenaRunArchive>(
                    callbacks, new_subs,
                ),
            "TBaseUnit" => client_cache
                .handle_resubscribe_for_type::<t_base_unit::TBaseUnit>(callbacks, new_subs),
            "TBattle" => {
                client_cache.handle_resubscribe_for_type::<t_battle::TBattle>(callbacks, new_subs)
            }
            "THouse" => {
                client_cache.handle_resubscribe_for_type::<t_house::THouse>(callbacks, new_subs)
            }
            "TRepresentation" => client_cache
                .handle_resubscribe_for_type::<t_representation::TRepresentation>(
                    callbacks, new_subs,
                ),
            "TStatus" => {
                client_cache.handle_resubscribe_for_type::<t_status::TStatus>(callbacks, new_subs)
            }
            "TTeam" => {
                client_cache.handle_resubscribe_for_type::<t_team::TTeam>(callbacks, new_subs)
            }
            "TUser" => {
                client_cache.handle_resubscribe_for_type::<t_user::TUser>(callbacks, new_subs)
            }
            _ => {
                spacetimedb_sdk::log::error!("TableRowOperation on unknown table {:?}", table_name)
            }
        }
    }
}

/// Connect to a database named `db_name` accessible over the internet at the URI `spacetimedb_uri`.
///
/// If `credentials` are supplied, they will be passed to the new connection to
/// identify and authenticate the user. Otherwise, a set of `Credentials` will be
/// generated by the server.
pub fn connect<IntoUri>(
    spacetimedb_uri: IntoUri,
    db_name: &str,
    credentials: Option<Credentials>,
) -> Result<()>
where
    IntoUri: TryInto<spacetimedb_sdk::http::Uri>,
    <IntoUri as TryInto<spacetimedb_sdk::http::Uri>>::Error:
        std::error::Error + Send + Sync + 'static,
{
    with_connection_mut(|connection| {
        connection.connect(spacetimedb_uri, db_name, credentials, Arc::new(Module))?;
        Ok(())
    })
}
