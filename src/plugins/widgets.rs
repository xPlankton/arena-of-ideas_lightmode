use egui::Area;

use super::*;

pub struct WidgetsPlugin;

impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::ui)
            .add_systems(Startup, Self::setup);
    }
}

impl WidgetsPlugin {
    fn setup(world: &mut World) {
        let Some(ctx) = &egui_context(world) else {
            return;
        };
        ctx.flip_name_enabled("Playback");
    }
    fn ui(world: &mut World) {
        let Some(ctx) = &egui_context(world) else {
            return;
        };
        if just_pressed(KeyCode::Escape, world) {
            ctx.flip_name_enabled("Main Menu");
        }
        Area::new(Id::new("top_right_info"))
            .anchor(Align2::RIGHT_TOP, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                    ui.add_space(13.0);
                    if let Some(fps) = world
                        .resource::<DiagnosticsStore>()
                        .get(&FrameTimeDiagnosticsPlugin::FPS)
                    {
                        if let Some(fps) = fps.smoothed() {
                            ui.label(format!("fps: {fps:.0}"));
                        }
                    }
                    ui.label(format!("arena-of-ideas {VERSION}"));
                })
            });

        StateMenu::default().show(ctx, world);

        let state = cur_state(world);
        match state {
            GameState::Title => Tile::right("Main Menu")
                .title()
                .open()
                .content(|ui, world| {
                    format!("Welcome, {}!", LoginPlugin::user(world).name)
                        .cstr_cs(DARK_WHITE, CstrStyle::Heading2)
                        .label(ui);
                    br(ui);
                    Button::toggle_child("New Game".into()).ui(ui);
                    Button::toggle_child("Settings".into()).ui(ui);
                })
                .child(|ctx, world| {
                    Tile::right("New Game")
                        .title()
                        .close_btn()
                        .content(|ui, _| {
                            if ui.button("test").clicked() {
                                debug!("test");
                            }
                        })
                        .show(ctx, world);
                    Tile::right("Settings")
                        .title()
                        .close_btn()
                        .content(|ui, _| {
                            if ui.button("setting 1").clicked() {
                                debug!("Test click");
                            }
                            br(ui);
                            if ui.button("setting 2").clicked() {
                                debug!("Test click");
                            }
                            br(ui);
                        })
                        .show(ctx, world);
                })
                .show(ctx, world),
            GameState::Profile => Tile::right("Profile")
                .title()
                .open()
                .content(|ui, _| {
                    Button::toggle_child("Settings".into()).ui(ui);
                })
                .child(|ctx, world| {
                    Tile::right("Settings")
                        .title()
                        .close_btn()
                        .content(|ui, world| {
                            ProfilePlugin::settings_ui(ui, world);
                        })
                        .show(ctx, world);
                })
                .show(ctx, world),
            GameState::Shop => ShopPlugin::widgets(ctx, world),
            GameState::Battle => BattlePlugin::widgets(ctx, world),
            _ => {}
        }
        let mut wd = world.remove_resource::<WidgetData>().unwrap();
        CentralPanel::default()
            .frame(Frame::none())
            .show(ctx, |ui| match state {
                GameState::Shop => ShopPlugin::show_containers(&mut wd, ui, world),
                GameState::Connect => LoginPlugin::connect_ui(ui, world),
                GameState::Login | GameState::ForceLogin => LoginPlugin::login_ui(ui, world),
                _ => {}
            });
        match state {
            GameState::Shop => ShopPlugin::overlay_widgets(ctx, world),
            _ => {}
        }
        Notification::show_all(&wd, ctx, world);
        world.insert_resource(wd);
    }
}
