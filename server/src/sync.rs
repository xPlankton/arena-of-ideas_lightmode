use ability::TAbility;
use base_unit::BaseUnit;
use house::THouse;
use spacetimedb::TableType;
use status::TStatus;

use super::*;

#[spacetimedb(reducer)]
fn sync_all_assets(
    ctx: ReducerContext,
    gs: GlobalSettings,
    units: Vec<BaseUnit>,
    houses: Vec<THouse>,
    abilities: Vec<TAbility>,
    statuses: Vec<TStatus>,
) -> Result<(), String> {
    gs.replace();
    for unit in BaseUnit::iter() {
        unit.delete();
    }
    for unit in units {
        BaseUnit::insert(unit)?;
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
    GlobalData::register_sync();
    Ok(())
}
