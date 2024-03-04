use std::{fmt::Display, ops::Add};

use bevy::input::mouse::MouseButton;
use bevy_egui::egui::{DragValue, Frame, Key, ScrollArea, Sense, Shape};
use hex::encode;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::de::DeserializeOwned;

use super::*;

pub struct HeroEditorPlugin;

impl Plugin for HeroEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (Self::input, Self::ui.after(PanelsPlugin::ui)).run_if(in_state(GameState::HeroEditor)),
        )
        .add_systems(OnEnter(GameState::HeroEditor), Self::on_enter)
        .add_systems(OnExit(GameState::HeroEditor), Self::on_exit);
    }
}

impl HeroEditorPlugin {
    fn on_enter(world: &mut World) {
        let mut pd = PersistentData::load(world);
        PackedTeam::spawn(Faction::Left, world);
        PackedTeam::spawn(Faction::Right, world);
        pd.hero_editor_data.apply_camera(world);
        pd.hero_editor_data.load(world);
        pd.save(world).unwrap();
    }

    fn on_exit(world: &mut World) {
        Self::save(world);
        Self::clear(world);
    }

    fn input(world: &mut World) {
        if world
            .resource::<Input<KeyCode>>()
            .just_pressed(KeyCode::Escape)
        {
            debug!("Close");
            let mut pd = PersistentData::load(world);
            let ed = &mut pd.hero_editor_data;
            ed.units.values_mut().for_each(|u| u.active = false);
            pd.save(world).unwrap();
        }
    }

    fn save(world: &mut World) {
        debug!("Saving.");
        let mut pd = PersistentData::load(world);
        pd.hero_editor_data.save(world);
        pd.save(world).unwrap();
    }

    fn ui(world: &mut World) {
        let mut pd = PersistentData::load(world);
        let ed = &mut pd.hero_editor_data;
        let ctx = &egui_context(world);
        let hovered = UnitPlugin::get_hovered(world);
        let mut delete: Option<Entity> = None;
        for (unit, data) in ed.units.iter_mut() {
            let unit = *unit;
            let hovered = hovered == Some(unit);
            if data.active || hovered {
                entity_window(unit, vec2(0.0, 0.0), None, &format!("{unit:?}"), world)
                    .frame(Frame::none())
                    .show(ctx, |ui| {
                        if hovered {
                            let button = ui.button_or_primary("EDIT", data.active);
                            if button.clicked() {
                                data.active = !data.active;
                            }
                            ui.add_space(5.0);
                            if ui.button_red("DELETE").clicked() {
                                delete = Some(unit);
                            }
                        }
                        if data.active {
                            data.show_window(unit, ui, world);
                        }
                    });
            }
        }
        if let Some(unit) = delete {
            ed.units.remove(&unit);
            world.entity_mut(unit).despawn_recursive();
            UnitPlugin::fill_gaps_and_translate(world);
        }
        for faction in [Faction::Left, Faction::Right] {
            let offset: Vec2 = match faction {
                Faction::Left => [-1.0, 0.0],
                _ => [1.0, 0.0],
            }
            .into();
            window(&format!("spawn {faction}"))
                .fixed_pos(world_to_screen(
                    (UnitPlugin::get_slot_position(faction, 0) + offset).extend(0.0),
                    world,
                ))
                .title_bar(false)
                .stroke(false)
                .set_width(60.0)
                .show(ctx, |ui| {
                    if ui.button("SPAWN").clicked() {
                        ed.spawn(faction, world);
                    }
                });
        }
        if world
            .resource::<Input<MouseButton>>()
            .get_just_released()
            .len()
            > 0
            || world.resource::<Input<KeyCode>>().get_just_released().len() > 0
        {
            pd.hero_editor_data.save(world);
        }
        pd.save(world).unwrap();
    }

    fn clear(world: &mut World) {
        let mut pd = PersistentData::load(world);
        let ed = &mut pd.hero_editor_data;
        UnitPlugin::despawn_all_teams(world);
        ed.clear();
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HeroEditorData {
    pub units: HashMap<Entity, EditorEntityData>,
    pub saved_units: (Vec<PackedUnit>, Vec<PackedUnit>),

    pub camera_pos: Vec2,
    pub camera_scale: f32,
    pub lookup: String,
    pub hovered_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct EditorEntityData {
    active: bool,
    window_center: egui::Vec2,
}

impl Default for HeroEditorData {
    fn default() -> Self {
        Self {
            units: default(),
            saved_units: default(),
            camera_pos: default(),
            camera_scale: 1.0,
            lookup: default(),
            hovered_id: default(),
        }
    }
}

impl EditorEntityData {
    fn show_window(&mut self, unit: Entity, ui: &mut Ui, world: &mut World) {
        if !self.active {
            return;
        }
        let pos = entity_screen_pos(unit, vec2(0.0, -1.0), world).to_pos2();
        let center = self.window_center;
        let ctx = &egui_context(world);
        CentralPanel::default()
            .frame(Frame::none())
            .show(ctx, |ui| {
                let end = center.to_pos2();
                let start = pos;
                ui.painter().line_segment(
                    [start, end],
                    Stroke {
                        width: 2.0,
                        color: white(),
                    },
                );
            });
        let unit_window = window(&format!("edit {unit:?}"))
            .default_pos(pos.add(egui::vec2(150.0, 0.0)))
            .title_bar(false)
            .order(egui::Order::Foreground)
            .show(&egui_context(world), |ui| {
                ui.style_mut().override_text_style = Some(TextStyle::Small);
                frame(ui, |ui| {
                    let houses: HashMap<String, Color> = HashMap::from_iter(
                        Pools::get(world)
                            .houses
                            .iter()
                            .map(|(k, v)| (k.clone(), v.color.clone().into())),
                    );
                    let mut state = VarState::get_mut(unit, world);
                    ui.horizontal(|ui| {
                        let name = &mut state.get_string(VarName::Name).unwrap();
                        ui.label("name:");
                        if TextEdit::singleline(name)
                            .desired_width(60.0)
                            .ui(ui)
                            .changed()
                        {
                            state.init(VarName::Name, VarValue::String(name.to_owned()));
                        }
                        let atk = &mut state.get_int(VarName::Atk).unwrap();
                        ui.label("atk:");
                        if DragValue::new(atk).clamp_range(0..=99).ui(ui).changed() {
                            state.init(VarName::Atk, VarValue::Int(*atk));
                        }
                        let hp = &mut state.get_int(VarName::Hp).unwrap();
                        ui.label("hp:");
                        if DragValue::new(hp).clamp_range(0..=99).ui(ui).changed() {
                            state.init(VarName::Hp, VarValue::Int(*hp));
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("house:");
                        let house = &mut state.get_string(VarName::Houses).unwrap();
                        ComboBox::from_id_source("house")
                            .selected_text(house.clone())
                            .width(140.0)
                            .show_ui(ui, |ui| {
                                for (h, c) in houses {
                                    if ui.selectable_value(house, h.clone(), h.clone()).changed() {
                                        state.init(VarName::Houses, VarValue::String(h));
                                        state.init(VarName::HouseColor1, VarValue::Color(c));
                                    }
                                }
                            });
                    });
                    ui.horizontal(|ui| {
                        ui.label("desc:");
                        let description = &mut state.get_string(VarName::Description).unwrap();
                        if TextEdit::singleline(description)
                            .desired_width(ui.available_width().min(200.0))
                            .ui(ui)
                            .changed()
                        {
                            state.init(
                                VarName::Description,
                                VarValue::String(description.to_owned()),
                            );
                        }
                    });
                    let context = &Context::from_owner(unit, world);
                    ui.horizontal(|ui| {
                        let trigger = &mut default();
                        mem::swap(
                            trigger,
                            &mut Status::find_unit_status(unit, LOCAL_TRIGGER, world)
                                .unwrap()
                                .trigger,
                        );
                        match trigger {
                            Trigger::Fire {
                                trigger,
                                target,
                                effect,
                            } => {
                                CollapsingHeader::new("TRIGGER").default_open(true).show(
                                    ui,
                                    |ui| {
                                        ComboBox::from_id_source(unit)
                                            .selected_text(trigger.to_string())
                                            .wrap(false)
                                            .show_ui(ui, |ui| {
                                                for option in FireTrigger::iter() {
                                                    let text = option.to_string();
                                                    ui.selectable_value(trigger, option, text);
                                                }
                                            });
                                        match trigger {
                                            FireTrigger::List(list) => {
                                                ui.vertical(|ui| {
                                                    for (i, trigger) in list.iter_mut().enumerate()
                                                    {
                                                        ComboBox::from_id_source(
                                                            Id::new(unit).with(i),
                                                        )
                                                        .selected_text(trigger.to_string())
                                                        .show_ui(ui, |ui| {
                                                            for option in FireTrigger::iter() {
                                                                let text = option.to_string();
                                                                ui.selectable_value(
                                                                    trigger.as_mut(),
                                                                    option,
                                                                    text,
                                                                );
                                                            }
                                                        });
                                                    }
                                                    if ui.button("+").clicked() {
                                                        list.push(default());
                                                    }
                                                });
                                            }
                                            _ => {}
                                        }
                                    },
                                );
                                CollapsingHeader::new("TARGET")
                                    .default_open(true)
                                    .show(ui, |ui| {
                                        show_tree(target, context, ui, world);
                                    });

                                CollapsingHeader::new("EFFECT")
                                    .default_open(true)
                                    .show(ui, |ui| {
                                        show_tree(effect, context, ui, world);
                                    });
                            }
                            Trigger::Change { .. } => todo!(),
                            Trigger::List(_) => todo!(),
                        }

                        mem::swap(
                            trigger,
                            &mut Status::find_unit_status(unit, LOCAL_TRIGGER, world)
                                .unwrap()
                                .trigger,
                        );
                    });
                    ui.horizontal(|ui| {
                        let mut rep = default();
                        mem::swap(
                            &mut rep,
                            world.get_mut::<Representation>(unit).unwrap().as_mut(),
                        );

                        for child in rep.children.iter_mut() {
                            child.show_editor(context, ui, world);
                        }
                        if ui.button("+").clicked() {
                            rep.add_child(world);
                        }

                        mem::swap(
                            &mut rep,
                            world.get_mut::<Representation>(unit).unwrap().as_mut(),
                        );
                    });
                });
            })
            .response;
        let window_pos = unit_window.rect.center();

        self.window_center = window_pos.to_vec2();
    }
}

impl HeroEditorData {
    fn save(&mut self, world: &mut World) {
        debug!("Save hero editor data start");
        self.saved_units.0.clear();
        self.saved_units.1.clear();
        let mut units = UnitPlugin::collect_factions([Faction::Left, Faction::Right].into(), world);
        units.sort_by_key(|(e, _)| VarState::get(*e, world).get_int(VarName::Slot).unwrap());
        for (unit, faction) in units {
            let packed = PackedUnit::pack(unit, world);
            let units = match faction {
                Faction::Left => &mut self.saved_units.0,
                _ => &mut self.saved_units.1,
            };
            units.push(packed);
        }
    }

    fn load(&mut self, world: &mut World) {
        debug!("Load hero editor data start");
        self.units.clear();
        let left = PackedTeam::find_entity(Faction::Left, world).unwrap();
        self.saved_units.0.iter().rev().for_each(|u| {
            let e = u.clone().unpack(left, None, world);
            self.units.insert(
                e,
                EditorEntityData {
                    active: true,
                    ..default()
                },
            );
        });
        let right = PackedTeam::find_entity(Faction::Right, world).unwrap();
        self.saved_units.1.iter().rev().for_each(|u| {
            let e = u.clone().unpack(right, None, world);
            self.units.insert(e, default());
        });
        UnitPlugin::fill_gaps_and_translate(world);
    }

    fn clear(&mut self) {
        self.units.clear();
        self.lookup.clear();
        self.hovered_id = None;
    }

    fn apply_camera(&mut self, world: &mut World) {
        if let Ok((mut transform, mut projection)) = world
            .query_filtered::<(&mut Transform, &mut OrthographicProjection), With<Camera>>()
            .get_single_mut(world)
        {
            let delta = self.camera_pos * self.camera_scale / projection.scale;
            self.camera_pos = delta;
            let z = transform.translation.z;
            transform.translation = delta.extend(z);
            projection.scale = self.camera_scale;
        }
    }

    fn spawn(&mut self, faction: Faction, world: &mut World) {
        let unit = ron::from_str::<PackedUnit>("()").unwrap();
        let unit = unit.unpack(
            PackedTeam::find_entity(faction, world).unwrap(),
            None,
            world,
        );
        UnitPlugin::fill_slot_gaps(faction, world);
        UnitPlugin::translate_to_slots(world);
        self.units.insert(unit, default());
    }
}

fn show_value(value: &Result<VarValue>, ui: &mut Ui) {
    match &value {
        Ok(v) => {
            v.to_string()
                .add_color(light_gray())
                .set_style(ColoredStringStyle::Small)
                .label(ui);
        }
        Err(e) => {
            e.to_string()
                .add_color(red())
                .set_style(ColoredStringStyle::Small)
                .as_label(ui)
                .truncate(true)
                .ui(ui);
        }
    }
}

pub fn show_tree(
    root: &mut impl EditorNodeGenerator,
    context: &Context,
    ui: &mut Ui,
    world: &mut World,
) {
    let style = ui.style_mut();
    style.override_text_style = Some(TextStyle::Small);
    style.drag_value_text_style = TextStyle::Small;
    style.visuals.widgets.inactive.bg_stroke = Stroke {
        width: 1.0,
        color: dark_gray(),
    };
    ui.horizontal(|ui| {
        show_node(root, default(), None, context, ui, world);
    });
}

fn show_node(
    source: &mut impl EditorNodeGenerator,
    path: String,
    connect_pos: Option<Pos2>,
    context: &Context,
    ui: &mut Ui,
    world: &mut World,
) {
    let path = format!("{path}/{source}");
    let ctx = &egui_context(world);
    let InnerResponse {
        inner: name_resp,
        response: frame_resp,
    } = Frame::none()
        .stroke(Stroke::new(1.0, dark_gray()))
        .inner_margin(6.0)
        .outer_margin(6.0)
        .rounding(0.0)
        .show(ui, |ui| {
            ui.set_min_width(50.0);
            let name = source
                .to_string()
                .add_color(source.node_color())
                .as_label(ui)
                .sense(Sense::click())
                .ui(ui);
            ui.allocate_ui_at_rect(
                name.rect.translate(egui::vec2(0.0, name.rect.height())),
                |ui| {
                    source.show_extra(&path, context, world, ui);
                },
            );
            name.on_hover_text(&path)
        });

    if name_resp.clicked() {
        name_resp.request_focus();
    }
    if name_resp.has_focus() || name_resp.lost_focus() {
        const LOOKUP_KEY: &str = "lookup";
        window("replace")
            .order(egui::Order::Foreground)
            .title_bar(false)
            .fixed_pos(frame_resp.rect.right_center().to_bvec2())
            .show(ctx, |ui| {
                Frame::none().inner_margin(8.0).show(ui, |ui| {
                    let mut lookup = get_context_string(world, LOOKUP_KEY);
                    let mut submit = false;
                    ctx.input(|i| {
                        for e in &i.events {
                            match e {
                                egui::Event::Text(t) => lookup += t,
                                egui::Event::Key { key, pressed, .. } => {
                                    if *pressed {
                                        if key.eq(&Key::Backspace) && !lookup.is_empty() {
                                            lookup.remove(lookup.len() - 1);
                                        } else if matches!(key, Key::Enter | Key::Tab) {
                                            submit = true;
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    });
                    ui.label(&lookup);
                    set_context_string(world, LOOKUP_KEY, lookup.clone());
                    ScrollArea::new([false, true])
                        .max_height(300.0)
                        .show(ui, |ui| {
                            let lookup = lookup.to_lowercase();
                            frame(ui, |ui| {
                                if source.show_replace_buttons(&lookup, submit, ui) {
                                    set_context_string(world, LOOKUP_KEY, default());
                                }
                            });
                        });
                });
            });
    }

    if let Some(pos) = connect_pos {
        let end = frame_resp.rect.left_center();
        let mut mid1 = pos;
        mid1.x += 5.0;
        let mut mid2 = end;
        mid2.x -= 5.0;
        let points = [pos, mid1, mid2, end];
        let curve = Shape::CubicBezier(egui::epaint::CubicBezierShape::from_points_stroke(
            points,
            false,
            Color32::TRANSPARENT,
            Stroke {
                width: 1.0,
                color: dark_gray(),
            },
        ));
        ui.painter().add(curve);
    }

    source.show_children(
        &path,
        Some(frame_resp.rect.right_center()),
        context,
        ui,
        world,
    );

    name_resp.context_menu(|ui| {
        if ui.button("COPY").clicked() {
            save_to_clipboard(
                &to_string_pretty(source, PrettyConfig::new()).unwrap(),
                world,
            );
            ui.close_menu();
        }
        if ui.button("PASTE").clicked() {
            let o = get_from_clipboard(world).unwrap();
            *source = ron::from_str(o.as_str()).unwrap();
            ui.close_menu();
        }
    });
}

pub trait EditorNodeGenerator: Display + Sized + Serialize + DeserializeOwned {
    fn node_color(&self) -> Color32;
    fn show_children(
        &mut self,
        path: &str,
        connect_pos: Option<Pos2>,
        context: &Context,
        ui: &mut Ui,
        world: &mut World,
    );
    fn show_extra(&mut self, path: &str, context: &Context, world: &mut World, ui: &mut Ui);
    fn show_replace_buttons(&mut self, lookup: &str, submit: bool, ui: &mut Ui) -> bool;
}

impl EditorNodeGenerator for Expression {
    fn node_color(&self) -> Color32 {
        self.editor_color()
    }

    fn show_extra(&mut self, path: &str, context: &Context, world: &mut World, ui: &mut Ui) {
        let value = self.get_value(context, world);
        match self {
            Expression::Value(v) => {
                ui.label(format!("{v:?}"));
            }
            Expression::Float(x) => {
                ui.add(DragValue::new(x).speed(0.1));
            }
            Expression::Int(x) => {
                ui.add(DragValue::new(x));
            }
            Expression::Bool(x) => {
                ui.checkbox(x, "");
            }
            Expression::String(x) => {
                ui.text_edit_singleline(x);
            }
            Expression::Hex(x) => {
                let c = Color::hex(&x).unwrap_or_default().as_rgba_u8();
                let mut c = Color32::from_rgb(c[0], c[1], c[2]);
                if ui.color_edit_button_srgba(&mut c).changed() {
                    *x = encode(c.to_array());
                }
            }
            Expression::Faction(x) => {
                ComboBox::from_id_source(&path)
                    .selected_text(x.to_string())
                    .show_ui(ui, |ui| {
                        for option in Faction::iter() {
                            let text = option.to_string();
                            ui.selectable_value(x, option, text).changed();
                        }
                    });
            }
            Expression::State(x)
            | Expression::TargetState(x)
            | Expression::Context(x)
            | Expression::StateLast(x) => {
                ComboBox::from_id_source(&path)
                    .selected_text(x.to_string())
                    .show_ui(ui, |ui| {
                        for option in VarName::iter() {
                            if context.get_var(option, world).is_some() {
                                let text = option.to_string();
                                ui.selectable_value(x, option, text).changed();
                            }
                        }
                    });
            }
            Expression::WithVar(x, ..) => {
                ui.vertical(|ui| {
                    ComboBox::from_id_source(&path)
                        .selected_text(x.to_string())
                        .show_ui(ui, |ui| {
                            for option in VarName::iter() {
                                let text = option.to_string();
                                ui.selectable_value(x, option, text).changed();
                            }
                        });
                    show_value(&value, ui);
                });
            }
            Expression::Vec2(x, y) => {
                ui.add(DragValue::new(x).speed(0.1));
                ui.add(DragValue::new(y).speed(0.1));
            }
            _ => show_value(&value, ui),
        };
    }

    fn show_children(
        &mut self,
        path: &str,
        connect_pos: Option<Pos2>,
        context: &Context,
        ui: &mut Ui,
        world: &mut World,
    ) {
        match self {
            Expression::Zero
            | Expression::GameTime
            | Expression::RandomFloat
            | Expression::PI
            | Expression::Age
            | Expression::SlotPosition
            | Expression::OwnerFaction
            | Expression::OppositeFaction
            | Expression::Beat
            | Expression::Owner
            | Expression::Caster
            | Expression::Target
            | Expression::RandomUnit
            | Expression::RandomAdjacentUnit
            | Expression::RandomAlly
            | Expression::RandomEnemy
            | Expression::AllyUnits
            | Expression::EnemyUnits
            | Expression::AllUnits
            | Expression::AdjacentUnits
            | Expression::Float(_)
            | Expression::Int(_)
            | Expression::Bool(_)
            | Expression::String(_)
            | Expression::Hex(_)
            | Expression::Faction(_)
            | Expression::State(_)
            | Expression::TargetState(_)
            | Expression::StateLast(_)
            | Expression::Context(_)
            | Expression::Value(_)
            | Expression::Vec2(_, _)
            | Expression::Vec2E(_)
            | Expression::StringInt(_)
            | Expression::StringFloat(_)
            | Expression::StringVec(_)
            | Expression::IntFloat(_) => default(),
            Expression::Sin(x)
            | Expression::Cos(x)
            | Expression::Sign(x)
            | Expression::Fract(x)
            | Expression::Floor(x)
            | Expression::UnitVec(x)
            | Expression::Even(x)
            | Expression::Abs(x)
            | Expression::SlotUnit(x)
            | Expression::FactionCount(x)
            | Expression::StatusCharges(x) => show_node(
                x.as_mut(),
                format!("{path}:x"),
                connect_pos,
                context,
                ui,
                world,
            ),

            Expression::Sum(a, b)
            | Expression::Sub(a, b)
            | Expression::Mul(a, b)
            | Expression::Div(a, b)
            | Expression::GreaterThen(a, b)
            | Expression::LessThen(a, b)
            | Expression::Min(a, b)
            | Expression::Max(a, b)
            | Expression::Equals(a, b)
            | Expression::And(a, b)
            | Expression::Vec2EE(a, b)
            | Expression::Or(a, b) => {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        show_node(
                            a.as_mut(),
                            format!("{path}:a"),
                            connect_pos,
                            context,
                            ui,
                            world,
                        );
                    });
                    ui.horizontal(|ui| {
                        show_node(
                            b.as_mut(),
                            format!("{path}:b"),
                            connect_pos,
                            context,
                            ui,
                            world,
                        );
                    });
                });
            }
            Expression::If(i, t, e) => {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        show_node(
                            i.as_mut(),
                            format!("{path}:i"),
                            connect_pos,
                            context,
                            ui,
                            world,
                        );
                    });
                    ui.horizontal(|ui| {
                        show_node(
                            t.as_mut(),
                            format!("{path}:t"),
                            connect_pos,
                            context,
                            ui,
                            world,
                        );
                    });
                    ui.horizontal(|ui| {
                        show_node(
                            e.as_mut(),
                            format!("{path}:e"),
                            connect_pos,
                            context,
                            ui,
                            world,
                        );
                    });
                });
            }
            Expression::WithVar(_, val, e) => {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        show_node(
                            val.as_mut(),
                            format!("{path}:val"),
                            connect_pos,
                            context,
                            ui,
                            world,
                        );
                    });
                    ui.horizontal(|ui| {
                        show_node(
                            e.as_mut(),
                            format!("{path}:e"),
                            connect_pos,
                            context,
                            ui,
                            world,
                        );
                    });
                });
            }
        };
    }

    fn show_replace_buttons(&mut self, lookup: &str, submit: bool, ui: &mut Ui) -> bool {
        for e in Expression::iter() {
            if e.to_string().to_lowercase().contains(lookup) {
                let btn = e.to_string().add_color(e.node_color()).rich_text(ui);
                let btn = ui.button(btn);
                if btn.clicked() || submit {
                    btn.request_focus();
                }
                if btn.gained_focus() {
                    *self = e.set_inner(self.clone());
                    return true;
                }
            }
        }
        false
    }
}

impl EditorNodeGenerator for Effect {
    fn node_color(&self) -> Color32 {
        white()
    }

    fn show_extra(&mut self, path: &str, context: &Context, world: &mut World, ui: &mut Ui) {
        match self {
            Effect::AoeFaction(_, _)
            | Effect::WithTarget(_, _)
            | Effect::WithOwner(_, _)
            | Effect::Noop
            | Effect::Kill
            | Effect::FullCopy
            | Effect::RemoveLocalTrigger
            | Effect::Debug(_)
            | Effect::Text(_) => {}

            Effect::List(list) | Effect::ListSpread(list) => {
                if ui.button("CLEAR").clicked() {
                    list.clear()
                }
            }
            Effect::Damage(e) => {
                let mut v = e.is_some();
                if ui.checkbox(&mut v, "").changed() {
                    *e = match v {
                        true => Some(default()),
                        false => None,
                    };
                }
            }
            Effect::WithVar(x, e, _) => {
                ui.vertical(|ui| {
                    ComboBox::from_id_source(&path)
                        .selected_text(x.to_string())
                        .show_ui(ui, |ui| {
                            for option in VarName::iter() {
                                let text = option.to_string();
                                ui.selectable_value(x, option, text).changed();
                            }
                        });
                    let value = e.get_value(context, world);
                    show_value(&value, ui);
                });
            }
            Effect::UseAbility(name) => {
                ui.vertical(|ui| {
                    ComboBox::from_id_source(&path)
                        .selected_text(name.to_owned())
                        .show_ui(ui, |ui| {
                            for option in Pools::get(world).abilities.keys() {
                                let text = option.to_string();
                                ui.selectable_value(name, option.to_owned(), text).changed();
                            }
                        });
                });
            }
            Effect::AddStatus(name) => {
                ui.vertical(|ui| {
                    ComboBox::from_id_source(&path)
                        .selected_text(name.to_owned())
                        .show_ui(ui, |ui| {
                            for option in Pools::get(world).statuses.keys() {
                                let text = option.to_string();
                                ui.selectable_value(name, option.to_owned(), text).changed();
                            }
                        });
                });
            }
            Effect::Vfx(name) => {
                ui.vertical(|ui| {
                    ComboBox::from_id_source(&path)
                        .selected_text(name.to_owned())
                        .show_ui(ui, |ui| {
                            for option in Pools::get(world).vfx.keys() {
                                let text = option.to_string();
                                ui.selectable_value(name, option.to_owned(), text).changed();
                            }
                        });
                });
            }
            Effect::SendEvent(name) => {
                ui.vertical(|ui| {
                    ComboBox::from_id_source(&path)
                        .selected_text(name.to_string())
                        .show_ui(ui, |ui| {
                            for option in [Event::BattleStart, Event::TurnStart, Event::TurnEnd] {
                                let text = option.to_string();
                                ui.selectable_value(name, option, text).changed();
                            }
                        });
                });
            }
        }
    }

    fn show_replace_buttons(&mut self, lookup: &str, submit: bool, ui: &mut Ui) -> bool {
        for e in Effect::iter() {
            if e.to_string().to_lowercase().contains(lookup) {
                let btn = e.to_string().add_color(e.node_color()).rich_text(ui);
                let btn = ui.button(btn);
                if btn.clicked() || submit {
                    btn.request_focus();
                }
                if btn.gained_focus() {
                    *self = e;
                    return true;
                }
            }
        }
        false
    }

    fn show_children(
        &mut self,
        path: &str,
        connect_pos: Option<Pos2>,
        context: &Context,
        ui: &mut Ui,
        world: &mut World,
    ) {
        match self {
            Effect::Noop
            | Effect::Kill
            | Effect::FullCopy
            | Effect::UseAbility(_)
            | Effect::AddStatus(_)
            | Effect::Vfx(_)
            | Effect::SendEvent(_)
            | Effect::RemoveLocalTrigger
            | Effect::Debug(_) => {}

            Effect::Text(e) => show_node(e, format!("{path}:e"), connect_pos, context, ui, world),
            Effect::Damage(e) => {
                if let Some(e) = e {
                    show_node(e, format!("{path}:e"), connect_pos, context, ui, world);
                }
            }
            Effect::AoeFaction(e, eff) | Effect::WithTarget(e, eff) | Effect::WithOwner(e, eff) => {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        show_node(e, format!("{path}:e"), connect_pos, context, ui, world);
                    });
                    ui.horizontal(|ui| {
                        show_node(
                            eff.as_mut(),
                            format!("{path}:eff"),
                            connect_pos,
                            context,
                            ui,
                            world,
                        );
                    });
                });
            }
            Effect::List(list) => {
                ui.vertical(|ui| {
                    for eff in list.iter_mut() {
                        ui.horizontal(|ui| {
                            show_node(
                                eff.as_mut(),
                                format!("{path}:eff"),
                                connect_pos,
                                context,
                                ui,
                                world,
                            );
                        });
                    }
                    if ui.button("+").clicked() {
                        list.push(default());
                    }
                });
            }
            Effect::ListSpread(_) => todo!(),
            Effect::WithVar(_, _, _) => todo!(),
        };
    }
}
