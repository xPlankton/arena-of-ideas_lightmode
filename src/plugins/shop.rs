use super::*;

use bevy_egui::egui::text::LayoutJob;
use bevy_egui::egui::{self, Align2, FontFamily, FontId, TextFormat, WidgetText};
use bevy_egui::egui::{pos2, Button, Color32, RichText, Window};
use rand::seq::IteratorRandom;

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Shop), Self::on_enter)
            .add_systems(OnExit(GameState::Shop), Self::on_leave)
            .add_systems(
                OnTransition {
                    from: GameState::Battle,
                    to: GameState::Shop,
                },
                Self::level_finished,
            )
            .add_systems(PostUpdate, Self::input)
            .add_systems(Update, (Self::ui.run_if(in_state(GameState::Shop)),));
    }
}

impl ShopPlugin {
    pub const UNIT_PRICE: i32 = 3;
    pub const REROLL_PRICE: i32 = 1;

    fn on_enter(world: &mut World) {
        if let Ok(team) = Self::active_team(world) {
            team.unpack(Faction::Team, world);
        } else {
            PackedTeam::spawn(Faction::Team, world);
        }
        UnitPlugin::translate_to_slots(world);
        Self::fill_showcase(world);
        Self::change_g(10, world).unwrap();

        PersistentData::save_last_state(GameState::Shop, world);
    }

    fn level_finished(world: &mut World) {
        let mut save = Save::get(world).unwrap();
        save.current_level += 1;
        if save.current_level >= Options::get_initial_ladder(world).teams.len() {
            let team =
                RatingPlugin::generate_weakest_opponent(&Save::get(world).unwrap().team, world);
            save.add_ladder_level(team);
        }
        save.save(world).unwrap();
    }

    fn on_leave(world: &mut World) {
        Self::pack_active_team(world).unwrap();
        UnitPlugin::despawn_all(world);
        Self::clear_showcase(world);

        let left = Self::active_team(world).unwrap();
        let right = Ladder::current_level(world);
        BattlePlugin::load_teams(left, right, world);
    }

    fn input(world: &mut World) {
        if just_pressed(KeyCode::G, world) {
            Self::change_g(10, world).unwrap();
        }
    }

    fn fill_showcase(world: &mut World) {
        let mut units = Vec::default();
        for _ in 0..3 {
            let unit = Pools::get(world)
                .heroes
                .values()
                .choose(&mut rand::thread_rng())
                .unwrap()
                .clone();
            units.push(unit);
        }
        let team = PackedTeam::spawn(Faction::Shop, world);
        let units_len = units.len();
        for unit in units {
            let description = unit.description.to_owned();
            let unit = unit.unpack(team, None, world);
            world.entity_mut(unit).insert(ShopOffer {
                name: "Hero".to_owned(),
                description,
                price: Self::UNIT_PRICE,
                product: OfferProduct::Unit,
            });
        }
        UnitPlugin::fill_slot_gaps(Faction::Shop, world);
        UnitPlugin::translate_to_slots(world);

        for i in 1..3 {
            let pos = UnitPlugin::get_slot_position(Faction::Shop, units_len + i as usize);
            let status = Pools::get_status("Strength", world).clone();
            let name = status.name.to_owned();
            let description = status.description.to_owned();
            let charges = status.state.get_int(VarName::Charges).unwrap_or(1);
            let entity = status.unpack(None, world).unwrap();
            VarState::get_mut(entity, world).init(VarName::Position, VarValue::Vec2(pos));
            world.entity_mut(entity).insert(ShopOffer {
                product: OfferProduct::Status {
                    name: name.to_owned(),
                    charges,
                },
                name,
                description,
                price: 2,
            });
        }
    }

    fn clear_showcase(world: &mut World) {
        for entity in Self::all_offers(world) {
            world.entity_mut(entity).despawn_recursive();
        }
    }

    pub fn pack_active_team(world: &mut World) -> Result<()> {
        let team = PackedTeam::pack(Faction::Team, world);
        debug!("Active team saved.");
        Save::get(world)?
            .set_team(team)
            .save(world)
            .map_err(|e| anyhow!("{}", e.to_string()))
    }

    pub fn active_team(world: &mut World) -> Result<PackedTeam> {
        Ok(Save::get(world)?.team)
    }

    pub fn ui(world: &mut World) {
        let ctx = &egui_context(world);
        for entity in Self::all_offers(world) {
            ShopOffer::draw_buy_panel(entity, world);
        }
        if let Some(team_state) = PackedTeam::state(Faction::Team, world) {
            let g = team_state.get_int(VarName::G).unwrap_or_default();
            Window::new("Stats").show(&ctx, |ui| {
                ui.label(RichText::new(format!("G: {g}")).color(Color32::KHAKI));
            });
        }
        let pos = UnitPlugin::get_slot_position(Faction::Shop, 0);
        let pos = vec3(pos.x, pos.y, 0.0);
        let pos = world_to_screen(pos, world);
        Window::new("reroll")
            .fixed_pos(pos2(pos.x, pos.y))
            .collapsible(false)
            .title_bar(false)
            .resizable(false)
            .default_width(10.0)
            .show(ctx, |ui| {
                ui.set_enabled(Self::reroll_affordable(world));
                ui.vertical_centered(|ui| {
                    let btn = Button::new(
                        RichText::new(format!("-{}g", Self::REROLL_PRICE))
                            .size(20.0)
                            .color(hex_color!("#00E5FF"))
                            .text_style(egui::TextStyle::Button),
                    )
                    .min_size(egui::vec2(100.0, 0.0));
                    ui.label("Reroll");
                    if ui.add(btn).clicked() {
                        Self::buy_reroll(world).unwrap();
                    }
                })
            });
        Window::new("battle")
            .anchor(Align2::RIGHT_BOTTOM, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                if ui.button("Go").clicked() {
                    GameState::change(GameState::Battle, world);
                    GameTimer::get_mut(world).clear_save();
                    GameTimer::get_mut(world).reset();
                }
            });
    }

    pub fn reroll_affordable(world: &mut World) -> bool {
        Self::get_g(world) >= Self::REROLL_PRICE
    }
    pub fn can_afford(price: i32, world: &mut World) -> bool {
        Self::get_g(world) >= price
    }

    pub fn buy_unit(unit: Entity, world: &mut World) -> Result<()> {
        let team = PackedTeam::entity(Faction::Team, world).unwrap();
        world
            .entity_mut(unit)
            .set_parent(team)
            .remove::<ShopOffer>();
        VarState::push_back(unit, VarName::Slot, Change::new(VarValue::Int(0)), world);
        UnitPlugin::fill_slot_gaps(Faction::Team, world);
        UnitPlugin::translate_to_slots(world);
        Self::change_g(-Self::UNIT_PRICE, world)
    }

    pub fn buy_reroll(world: &mut World) -> Result<()> {
        Self::clear_showcase(world);
        Self::fill_showcase(world);
        Self::change_g(-Self::REROLL_PRICE, world)
    }

    pub fn get_g(world: &mut World) -> i32 {
        PackedTeam::state(Faction::Team, world)
            .and_then(|s| s.get_int(VarName::G).ok())
            .unwrap_or_default()
    }

    pub fn change_g(delta: i32, world: &mut World) -> Result<()> {
        debug!("Change g {delta}");
        VarState::change_int(
            PackedTeam::entity(Faction::Team, world).unwrap(),
            VarName::G,
            delta,
            world,
        )
    }

    pub fn all_offers(world: &mut World) -> Vec<Entity> {
        world
            .query_filtered::<Entity, With<ShopOffer>>()
            .iter(world)
            .collect_vec()
    }
}

#[derive(Component, Clone, Debug)]
pub struct ShopOffer {
    pub name: String,
    pub description: String,
    pub price: i32,
    pub product: OfferProduct,
}

#[derive(Clone, Debug)]
pub enum OfferProduct {
    Unit,
    Status { name: String, charges: i32 },
}

impl OfferProduct {
    pub fn do_buy(&self, entity: Entity, world: &mut World) -> Result<()> {
        match self {
            OfferProduct::Unit => ShopPlugin::buy_unit(entity, world),
            OfferProduct::Status { name, charges } => {
                for unit in UnitPlugin::collect_faction(Faction::Team, world) {
                    Status::change_charges(name, unit, *charges, world).unwrap();
                }
                world.entity_mut(entity).despawn_recursive();
                Ok(())
            }
        }
    }
}

impl ShopOffer {
    pub fn draw_buy_panel(entity: Entity, world: &mut World) {
        let so = world.get::<ShopOffer>(entity).unwrap().clone();
        let window = entity_panel(entity, vec2(0.0, -1.5), "buy_panel", world);
        let ctx = &egui_context(world);
        window.show(ctx, |ui: &mut egui::Ui| {
            ui.set_enabled(ShopPlugin::can_afford(so.price, world));
            ui.vertical_centered(|ui| {
                let btn = Button::new(
                    RichText::new(format!("-{}g", so.price))
                        .size(20.0)
                        .color(hex_color!("#00E5FF"))
                        .text_style(egui::TextStyle::Button),
                )
                .min_size(egui::vec2(100.0, 0.0));
                ui.label("Buy");
                if ui.add(btn).clicked() {
                    so.product.do_buy(entity, world).unwrap();
                }
            })
        });
        if !so.description.is_empty() {
            let color = VarState::get(entity, world)
                .get_color(VarName::HouseColor)
                .unwrap_or(Color::WHITE)
                .as_rgba_u8();
            let color = Color32::from_rgb(color[0], color[1], color[2]);
            let window = entity_panel(entity, vec2(0.0, 1.1), "desc_panel", world);
            window.show(ctx, |ui| {
                let mut source = so.description.clone();
                let mut lines: Vec<(String, Color32)> = default();
                while let Some(pos) = source.find("[") {
                    let left = &source[..pos];
                    let pos2 = source.find("]").unwrap();
                    let mid = &source[pos + 1..pos2];
                    if let Some(cursor_pos) = cursor_pos(world) {
                        let cursor_pos = [cursor_pos.x, cursor_pos.y];
                        let ability = Pools::get_ability(mid, world);
                        if world.resource::<HoveredUnit>().0 == Some(entity) {
                            Window::new(RichText::new(mid).color(color).strong())
                                .fixed_pos(cursor_pos)
                                .collapsible(false)
                                .show(ctx, |ui| {
                                    ui.label(&ability.description);
                                });
                        }
                    }

                    lines.push((left.to_owned(), Color32::WHITE));
                    lines.push((mid.to_owned(), color));
                    source = source[pos2 + 1..].to_owned();
                }
                lines.push((source, Color32::WHITE));

                let mut job = LayoutJob::default();
                for (text, color) in lines {
                    job.append(
                        &text,
                        0.0,
                        TextFormat {
                            font_id: FontId::new(14.0, FontFamily::Proportional),
                            color,
                            ..Default::default()
                        },
                    );
                }
                ui.label(WidgetText::LayoutJob(job));
            });
        }
    }
}
