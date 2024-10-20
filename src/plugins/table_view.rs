use super::*;

pub struct TableViewPlugin;

impl Plugin for TableViewPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TablesData>()
            .add_systems(
                OnEnter(GameState::TableView(StdbQuery::BattleHistory)),
                Self::on_enter_history,
            )
            .add_systems(
                OnEnter(GameState::TableView(StdbQuery::BaseUnits)),
                Self::on_enter_base_units,
            );
    }
}

#[derive(Resource, Default)]
struct TablesData {
    battles: Vec<TBattle>,
    base_units: Vec<TBaseUnit>,
}

impl TableViewPlugin {
    fn on_enter_history(mut data: ResMut<TablesData>) {
        data.battles = TBattle::iter()
            .sorted_by(|a, b| b.id.cmp(&a.id))
            .collect_vec();
    }
    fn on_enter_base_units(mut data: ResMut<TablesData>) {
        data.base_units = TBaseUnit::iter().collect_vec();
    }
    pub fn add_tiles(query: StdbQuery, world: &mut World) {
        match query {
            StdbQuery::BattleHistory => Self::add_battle_history_tile(world),
            StdbQuery::BaseUnits => Self::add_base_units_tile(world),
            _ => panic!("Query not supported {query}"),
        }
    }
    fn add_battle_history_tile(world: &mut World) {
        Tile::new(Side::Left, |ui, world| {
            let td = world.remove_resource::<TablesData>().unwrap();
            Table::new("Battle History")
                .title()
                .column_gid("id", |d: &TBattle| d.id)
                .column_cstr("mode", |d, _| d.mode.cstr())
                .column_user_click(
                    "player",
                    |d| d.owner,
                    |gid, _, world| TilePlugin::add_user(gid, world),
                )
                .column_team("player team >", |d| d.team_left)
                .column_team("< enemy team", |d| d.team_right)
                .column_user_click(
                    "enemy",
                    |d| d.team_right.get_team().owner,
                    |gid, _, world| TilePlugin::add_user(gid, world),
                )
                .column_cstr("result", |d, _| match d.result {
                    TBattleResult::Tbd => "-".cstr(),
                    TBattleResult::Left => "W".cstr_c(GREEN),
                    TBattleResult::Right | TBattleResult::Even => "L".cstr_c(RED),
                })
                .column_ts("time", |d| d.ts)
                .column_btn("copy", |d, _, world| {
                    copy_to_clipboard(
                        &ron::to_string(&BattleResource::from(d.clone())).unwrap(),
                        world,
                    );
                })
                .column_btn("editor", |d, _, world| {
                    EditorPlugin::load_battle(
                        PackedTeam::from_id(d.team_left),
                        PackedTeam::from_id(d.team_right),
                    );
                    GameState::Editor.set_next(world);
                })
                .column_btn("run", |d, _, world| {
                    world.insert_resource(BattleResource::from(d.clone()));
                    BattlePlugin::set_next_state(cur_state(world), world);
                    GameState::Battle.set_next(world);
                })
                .filter("My", "player", user_id().into())
                .filter("Win", "result", "W".into())
                .filter("Lose", "result", "L".into())
                .filter("TBD", "result", "-".into())
                .ui(&td.battles, ui, world);
            world.insert_resource(td);
        })
        .pinned()
        .push(world);
    }
    fn add_base_units_tile(world: &mut World) {
        Tile::new(Side::Left, |ui, world| {
            let td = world.remove_resource::<TablesData>().unwrap();

            td.base_units
                .show_modified_table("Base Units", ui, world, |t| {
                    t.column_btn("spawn", |u, _, world| {
                        let unit: PackedUnit = u.clone().into();
                        TeamPlugin::despawn(Faction::Team, world);
                        unit.unpack(TeamPlugin::entity(Faction::Team, world), None, None, world);
                    })
                });

            world.insert_resource(td);
        })
        .pinned()
        .push(world);
    }
}
