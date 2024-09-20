use ability::TAbility;
use base_unit::TBaseUnit;
use house::THouse;
use representation::TRepresentation;

use status::TStatus;

use super::*;

fn replace_assets(
    global_settings: GlobalSettings,
    representations: Vec<TRepresentation>,
    base_units: Vec<TBaseUnit>,
    houses: Vec<THouse>,
    abilities: Vec<TAbility>,
    statuses: Vec<TStatus>,
) -> Result<(), String> {
    global_settings.replace();
    for r in TRepresentation::iter() {
        r.delete();
    }
    for r in representations {
        TRepresentation::insert(r)?;
    }
    for unit in TBaseUnit::iter() {
        unit.delete();
    }
    for unit in base_units {
        TBaseUnit::insert(unit)?;
    }
    for house in THouse::iter() {
        house.delete();
    }
    for house in houses {
        THouse::insert(house)?;
    }
    for status in TStatus::iter() {
        status.delete();
    }
    for status in statuses {
        TStatus::insert(status)?;
    }
    for ability in TAbility::iter() {
        ability.delete();
    }
    for ability in abilities {
        TAbility::insert(ability)?;
    }
    let ghost = || FusedUnit::from_base_name(GlobalSettings::get().ghost_unit, next_id()).unwrap();
    let enemies = [
        TTeam::new(0, TeamPool::Enemy).units(vec![ghost()]).save(),
        TTeam::new(0, TeamPool::Enemy)
            .units(vec![ghost(), ghost()])
            .save(),
        TTeam::new(0, TeamPool::Enemy)
            .units(vec![ghost(), ghost(), ghost()])
            .save(),
    ]
    .into();
    GlobalData::set_initial_enemies(enemies);
    if GlobalData::get().last_sync.eq(&Timestamp::UNIX_EPOCH) {
        daily_timer_init();
    }
    GlobalData::register_sync();
    Ok(())
}

#[spacetimedb(reducer)]
fn upload_assets(
    ctx: ReducerContext,
    global_settings: GlobalSettings,
    representations: Vec<TRepresentation>,
    base_units: Vec<TBaseUnit>,
    houses: Vec<THouse>,
    abilities: Vec<TAbility>,
    statuses: Vec<TStatus>,
) -> Result<(), String> {
    ctx.is_admin()?;
    replace_assets(
        global_settings,
        representations,
        base_units,
        houses,
        abilities,
        statuses,
    )
}

#[spacetimedb(reducer)]
fn upload_game_archive(
    ctx: ReducerContext,
    global_settings: Option<GlobalSettings>,
    global_data: Option<GlobalData>,
    users: Vec<TUser>,
    base_units: Vec<TBaseUnit>,
    houses: Vec<THouse>,
    abilities: Vec<TAbility>,
    statuses: Vec<TStatus>,
    representations: Vec<TRepresentation>,
    arena_runs: Vec<TArenaRun>,
    arena_runs_archive: Vec<TArenaRunArchive>,
    arena_leaderboard: Vec<TArenaLeaderboard>,
    teams: Vec<TTeam>,
    battles: Vec<TBattle>,
) -> Result<(), String> {
    ctx.is_admin()?;
    if let Some(global_settings) = global_settings {
        replace_assets(
            global_settings,
            representations,
            base_units,
            houses,
            abilities,
            statuses,
        )?;
    }
    if let Some(data) = global_data {
        GlobalData::delete_by_always_zero(&0);
        GlobalData::insert(data)?;
    }
    if !users.is_empty() {
        for d in TUser::iter() {
            d.delete();
        }
        for d in users {
            TUser::insert(d)?;
        }
    }
    if !arena_runs.is_empty() {
        for d in TArenaRun::iter() {
            d.delete();
        }
        for d in arena_runs {
            TArenaRun::insert(d)?;
        }
    }
    if !arena_runs_archive.is_empty() {
        for d in TArenaRunArchive::iter() {
            d.delete();
        }
        for d in arena_runs_archive {
            TArenaRunArchive::insert(d)?;
        }
    }
    if !arena_leaderboard.is_empty() {
        for d in TArenaLeaderboard::iter() {
            d.delete();
        }
        for d in arena_leaderboard {
            TArenaLeaderboard::insert(d);
        }
    }
    if !teams.is_empty() {
        for d in TTeam::iter() {
            d.delete();
        }
        for d in teams {
            TTeam::insert(d)?;
        }
    }
    if !battles.is_empty() {
        for d in TBattle::iter() {
            d.delete();
        }
        for d in battles {
            TBattle::insert(d)?;
        }
    }

    Ok(())
}
