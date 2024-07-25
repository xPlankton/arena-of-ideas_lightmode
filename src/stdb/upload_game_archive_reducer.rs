// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused_imports)]use spacetimedb_sdk::{
	Address,
	sats::{ser::Serialize, de::Deserialize},
	table::{TableType, TableIter, TableWithPrimaryKey},
	reducer::{Reducer, ReducerCallbackId, Status},
	identity::Identity,
	spacetimedb_lib,
	anyhow::{Result, anyhow},
};
use super::t_user::TUser;
use super::t_team::TTeam;
use super::global_data::GlobalData;
use super::t_base_unit::TBaseUnit;
use super::t_arena_run_archive::TArenaRunArchive;
use super::t_ability::TAbility;
use super::t_arena_run::TArenaRun;
use super::t_battle::TBattle;
use super::global_settings::GlobalSettings;
use super::t_representation::TRepresentation;
use super::t_house::THouse;
use super::t_status::TStatus;
use super::t_arena_leaderboard::TArenaLeaderboard;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UploadGameArchiveArgs {
	pub global_settings: GlobalSettings,
	pub global_data: GlobalData,
	pub users: Vec::<TUser>,
	pub base_units: Vec::<TBaseUnit>,
	pub houses: Vec::<THouse>,
	pub abilities: Vec::<TAbility>,
	pub statuses: Vec::<TStatus>,
	pub representations: Vec::<TRepresentation>,
	pub arena_runs: Vec::<TArenaRun>,
	pub arena_runs_archive: Vec::<TArenaRunArchive>,
	pub arena_leaderboard: Vec::<TArenaLeaderboard>,
	pub teams: Vec::<TTeam>,
	pub battles: Vec::<TBattle>,
}

impl Reducer for UploadGameArchiveArgs {
	const REDUCER_NAME: &'static str = "upload_game_archive";
}

#[allow(unused)]
pub fn upload_game_archive(
	global_settings: GlobalSettings,
	global_data: GlobalData,
	users: Vec::<TUser>,
	base_units: Vec::<TBaseUnit>,
	houses: Vec::<THouse>,
	abilities: Vec::<TAbility>,
	statuses: Vec::<TStatus>,
	representations: Vec::<TRepresentation>,
	arena_runs: Vec::<TArenaRun>,
	arena_runs_archive: Vec::<TArenaRunArchive>,
	arena_leaderboard: Vec::<TArenaLeaderboard>,
	teams: Vec::<TTeam>,
	battles: Vec::<TBattle>,
) {
		UploadGameArchiveArgs {
		global_settings,
		global_data,
		users,
		base_units,
		houses,
		abilities,
		statuses,
		representations,
		arena_runs,
		arena_runs_archive,
		arena_leaderboard,
		teams,
		battles,
}	.invoke();
}

#[allow(unused)]
pub fn on_upload_game_archive(mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &GlobalSettings, &GlobalData, &Vec::<TUser>, &Vec::<TBaseUnit>, &Vec::<THouse>, &Vec::<TAbility>, &Vec::<TStatus>, &Vec::<TRepresentation>, &Vec::<TArenaRun>, &Vec::<TArenaRunArchive>, &Vec::<TArenaLeaderboard>, &Vec::<TTeam>, &Vec::<TBattle>) + Send + 'static) -> ReducerCallbackId<UploadGameArchiveArgs> 
{
		UploadGameArchiveArgs::on_reducer(move |__identity, __addr, __status, __args| {
		let UploadGameArchiveArgs {
			global_settings,
			global_data,
			users,
			base_units,
			houses,
			abilities,
			statuses,
			representations,
			arena_runs,
			arena_runs_archive,
			arena_leaderboard,
			teams,
			battles,
}		 = __args;
__callback(
						__identity,
			__addr,
			__status,
			global_settings,
			global_data,
			users,
			base_units,
			houses,
			abilities,
			statuses,
			representations,
			arena_runs,
			arena_runs_archive,
			arena_leaderboard,
			teams,
			battles,
);
})
}

#[allow(unused)]
pub fn once_on_upload_game_archive(__callback: impl FnOnce(&Identity, Option<Address>, &Status, &GlobalSettings, &GlobalData, &Vec::<TUser>, &Vec::<TBaseUnit>, &Vec::<THouse>, &Vec::<TAbility>, &Vec::<TStatus>, &Vec::<TRepresentation>, &Vec::<TArenaRun>, &Vec::<TArenaRunArchive>, &Vec::<TArenaLeaderboard>, &Vec::<TTeam>, &Vec::<TBattle>) + Send + 'static) -> ReducerCallbackId<UploadGameArchiveArgs> 
{
		UploadGameArchiveArgs::once_on_reducer(move |__identity, __addr, __status, __args| {
		let UploadGameArchiveArgs {
			global_settings,
			global_data,
			users,
			base_units,
			houses,
			abilities,
			statuses,
			representations,
			arena_runs,
			arena_runs_archive,
			arena_leaderboard,
			teams,
			battles,
}		 = __args;
__callback(
						__identity,
			__addr,
			__status,
			global_settings,
			global_data,
			users,
			base_units,
			houses,
			abilities,
			statuses,
			representations,
			arena_runs,
			arena_runs_archive,
			arena_leaderboard,
			teams,
			battles,
);
})
}

#[allow(unused)]
pub fn remove_on_upload_game_archive(id: ReducerCallbackId<UploadGameArchiveArgs>) {
	UploadGameArchiveArgs::remove_on_reducer(id);
}
