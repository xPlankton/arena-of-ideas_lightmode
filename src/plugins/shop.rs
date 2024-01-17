use crate::module_bindings::{
    run_buy, run_change_g, run_reroll, run_sell, run_submit_result, ArenaPool, ArenaRun, TeamUnit,
};

use super::*;

use bevy::input::common_conditions::input_just_pressed;

pub struct ShopPlugin;

#[derive(Resource, Clone)]
struct ShopData {
    bottom_expanded: bool,
    update_callback: UpdateCallbackId<ArenaRun>,
}

const REROLL_PRICE: i32 = 1;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Shop), Self::on_enter)
            .add_systems(OnExit(GameState::Shop), Self::on_exit)
            .add_systems(
                OnTransition {
                    from: GameState::Shop,
                    to: GameState::Battle,
                },
                Self::transition_to_battle,
            )
            .add_systems(PostUpdate, Self::input.run_if(in_state(GameState::Shop)))
            .add_systems(
                Update,
                ((
                    Self::ui.after(PanelsPlugin::ui),
                    Self::win.run_if(input_just_pressed(KeyCode::V)),
                )
                    .run_if(in_state(GameState::Shop)),),
            );
    }
}

impl ShopPlugin {
    fn win(world: &mut World) {
        Self::transition_to_battle(world);
        Self::on_enter(world);
        run_submit_result(true);
    }

    fn on_enter(world: &mut World) {
        GameTimer::get().reset();
        PackedTeam::spawn(Faction::Shop, world);
        PackedTeam::spawn(Faction::Team, world);
        let update_callback = ArenaRun::on_update(|_, new, _| {
            let new = new.clone();
            OperationsPlugin::add(move |world| {
                Self::sync_units(&new.state.team, Faction::Team, world);
                Self::sync_units_state(&new.state.team, Faction::Team, world);
                Self::sync_units(
                    &new.state
                        .case
                        .into_iter()
                        .filter_map(|o| if o.available { Some(o.unit) } else { None })
                        .collect_vec(),
                    Faction::Shop,
                    world,
                );
            })
        });
        egui_context(world).data_mut(|w| w.clear());
        run_reroll(true);
        UnitPlugin::translate_to_slots(world);
        ActionPlugin::set_timeframe(0.05, world);

        world.insert_resource(ShopData {
            bottom_expanded: false,
            update_callback,
        });
    }

    fn on_exit(world: &mut World) {
        UnitPlugin::despawn_all_teams(world);
        Representation::despawn_all(world);
        ArenaRun::remove_on_update(world.resource::<ShopData>().update_callback.clone());
    }

    fn transition_to_battle(world: &mut World) {
        let run = ArenaRun::filter_by_active(true).next().unwrap();
        let left =
            PackedTeam::from_table_units(run.state.team.into_iter().map(|u| u.unit).collect());
        let right = run.enemies.last().unwrap();
        let right = ArenaPool::filter_by_id(*right).unwrap().team;
        let right = PackedTeam::from_table_units(right);
        BattlePlugin::load_teams(left, right, world);
    }

    fn input(world: &mut World) {
        if just_pressed(KeyCode::G, world) {
            run_change_g(10);
        }
    }

    fn sync_units(units: &Vec<TeamUnit>, faction: Faction, world: &mut World) {
        debug!("Start sync {} {faction}", units.len());
        let world_units = UnitPlugin::collect_faction_ids(faction, world);
        let team = PackedTeam::find_entity(faction, world).unwrap();
        for unit in units {
            if world_units.contains_key(&unit.id) {
                continue;
            }
            let id = unit.id;
            let unit: PackedUnit = unit.unit.clone().into();
            let unit = unit.unpack(team, None, world);
            VarState::get_mut(unit, world).set_int(VarName::Id, id as i32);
        }
        let world_units = UnitPlugin::collect_faction(faction, world);
        if world_units.len() > units.len() {
            for unit in world_units {
                let id = VarState::get(unit, world).get_int(VarName::Id).unwrap() as u64;
                if units.iter().find(|u| u.id.eq(&id)).is_none() {
                    world.entity_mut(unit).despawn_recursive();
                }
            }
        }
        UnitPlugin::fill_slot_gaps(faction, world);
        UnitPlugin::translate_to_slots(world);
    }

    fn sync_units_state(units: &Vec<TeamUnit>, faction: Faction, world: &mut World) {
        let world_units = UnitPlugin::collect_faction_ids(faction, world);
        for TeamUnit { id, unit } in units {
            let entity = world_units.get(id).unwrap();
            let mut state = VarState::get_mut(*entity, world);
            state.set_int(VarName::Hp, unit.hp);
            state.set_int(VarName::Atk, unit.atk);
            state.set_int(VarName::Stacks, unit.stacks);
            state.set_int(VarName::Level, unit.level);
            state.set_string(VarName::Description, unit.description.clone());
            state.set_string(VarName::House, unit.house.clone());
        }
    }

    pub fn ui(world: &mut World) {
        let ctx = &egui_context(world);
        let data = world.remove_resource::<ShopData>().unwrap();

        let pos = UnitPlugin::get_slot_position(Faction::Shop, 0) - vec2(1.0, 0.0);
        let pos = world_to_screen(pos.extend(0.0), world);
        let pos = pos2(pos.x, pos.y);

        if !data.bottom_expanded {
            Self::draw_buy_panels(world);
            let _ = Self::show_hero_ui(world);
        }
        Area::new("reroll").fixed_pos(pos).show(ctx, |ui| {
            ui.set_width(120.0);
            frame(ui, |ui| {
                ui.label("Reroll".add_color(white()).rich_text());
                if ui
                    .button(
                        format!("-{}g", REROLL_PRICE)
                            .add_color(yellow())
                            .rich_text()
                            .size(20.0),
                    )
                    .clicked()
                {
                    Self::buy_reroll();
                }
            });
        });

        let g = ArenaRun::filter_by_active(true).next().unwrap().state.g;
        Area::new("g")
            .fixed_pos(pos + egui::vec2(0.0, -60.0))
            .show(ctx, |ui| {
                ui.label(
                    RichText::new(format!("{g} g"))
                        .size(40.0)
                        .strong()
                        .color(hex_color!("#FFC107")),
                );
            });
        Area::new("battle button")
            .anchor(Align2::RIGHT_CENTER, [-40.0, 0.0])
            .show(ctx, |ui| {
                if ui.button("START BATTLE").clicked() {
                    Self::go_to_battle(world);
                }
            });
        world.insert_resource(data);
    }

    fn show_hero_ui(world: &mut World) -> Result<()> {
        let ctx = &egui_context(world);
        let cursor_pos = CameraPlugin::cursor_world_pos(world).context("Failed to get cursor")?;
        let dragged = world.resource::<DraggedUnit>().0;
        if let Some(dragged) = dragged {
            let dragged_state = VarState::get(dragged, world);
            let dragged_name = dragged_state.get_string(VarName::Name)?;
            for entity in UnitPlugin::collect_faction(Faction::Team, world) {
                if entity == dragged {
                    continue;
                }
                let state = VarState::get(entity, world);
                let same_slot = state.get_int(VarName::Slot).context("Failed to get slot")?
                    == UnitPlugin::get_closest_slot(cursor_pos, Faction::Team).0 as i32;
                let same_name = dragged_name.eq(&state.get_string(VarName::Name)?);
                if same_name {
                    let stacks = state.get_int(VarName::Stacks)?;
                    let level = state.get_int(VarName::Level)?;
                    let color = if same_slot { yellow() } else { white() };
                    window("STACK")
                        .id(entity)
                        .set_width(150.0)
                        .title_bar(false)
                        .stroke(false)
                        .entity_anchor(entity, Align2::CENTER_BOTTOM, vec2(0.0, 2.2), world)
                        .show(ctx, |ui| {
                            frame(ui, |ui| {
                                ui.label("+STACK".add_color(color).rich_text().size(24.0));
                                ui.label(format!("Level {level}").add_color(color).rich_text());
                                ui.label(
                                    format!("{stacks}/{}", level + 1)
                                        .add_color(light_gray())
                                        .rich_text(),
                                );
                            });
                        });
                }
            }
        } else {
            for entity in UnitPlugin::collect_faction(Faction::Team, world) {
                let state = VarState::get(entity, world);
                if state.get_int(VarName::Slot).context("Failed to get slot")?
                    == UnitPlugin::get_closest_slot(cursor_pos, Faction::Team).0 as i32
                {
                    window("SELL")
                        .id(entity)
                        .set_width(120.0)
                        .title_bar(false)
                        .stroke(false)
                        .entity_anchor(entity, Align2::CENTER_BOTTOM, vec2(0.0, 2.0), world)
                        .show(ctx, |ui| {
                            frame(ui, |ui| {
                                ui.set_width(100.0);
                                ui.label("sell");
                                if ui
                                    .button("+1 g".add_color(yellow()).rich_text().size(20.0))
                                    .clicked()
                                {
                                    run_sell(
                                        VarState::get(entity, world).get_int(VarName::Id).unwrap()
                                            as u64,
                                    );
                                    world.entity_mut(entity).despawn_recursive();
                                    UnitPlugin::fill_slot_gaps(Faction::Team, world);
                                    UnitPlugin::translate_to_slots(world);
                                }
                            });
                        });
                }
            }
        }

        Ok(())
    }
    fn draw_buy_panels(world: &mut World) {
        let ctx = &egui_context(world);
        let run = ArenaRun::filter_by_active(true).next().unwrap();
        let units = HashMap::from_iter(
            UnitPlugin::collect_faction(Faction::Shop, world)
                .into_iter()
                .map(|unit| {
                    (
                        VarState::get(unit, world).get_int(VarName::Id).unwrap() as u64,
                        unit,
                    )
                }),
        );
        for offer in run.state.case {
            let id = offer.unit.id;

            if let Some(entity) = units.get(&id) {
                window("BUY")
                    .id(&entity)
                    .set_width(120.0)
                    .title_bar(false)
                    .stroke(false)
                    .entity_anchor(*entity, Align2::CENTER_TOP, vec2(0.0, -1.2), world)
                    .show(ctx, |ui| {
                        // ui.set_enabled(
                        //     offer.available
                        //         && save.climb.shop.can_afford(offer.price)
                        //         && save.climb.team.units.len() < TEAM_SLOTS,
                        // );
                        frame(ui, |ui| {
                            ui.set_width(100.0);
                            if ui
                                .button(
                                    format!("-{} g", offer.price)
                                        .add_color(yellow())
                                        .rich_text()
                                        .size(20.0),
                                )
                                .clicked()
                            {
                                run_buy(id);
                            }
                        });
                    });
            }
        }
    }

    fn go_to_battle(world: &mut World) {
        GameState::change(GameState::Battle, world);
    }

    pub fn buy_reroll() {
        run_reroll(false);
    }
}
