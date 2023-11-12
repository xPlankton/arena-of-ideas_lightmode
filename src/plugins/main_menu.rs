use super::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::ui.run_if(in_state(GameState::MainMenu)));
    }
}

impl MainMenuPlugin {
    pub fn ui(world: &mut World) {
        let ctx = &egui_context(world);
        Window::new("Menu")
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    let btn = Self::menu_button("Shop".to_owned(), ui);
                    if ui.add(btn).clicked() {
                        GameState::change(GameState::Shop, world);
                    }
                    let btn = Self::menu_button("Custom Battle".to_owned(), ui);
                    if ui.add(btn).clicked() {
                        GameState::change(GameState::CustomBattle, world);
                    }
                    let btn = Self::menu_button("Run Tests".to_owned(), ui);
                    if ui.add(btn).clicked() {
                        GameState::change(GameState::TestsLoading, world);
                    }
                    ui.add_space(15.0);
                });
            });
    }

    fn menu_button(name: String, ui: &mut Ui) -> Button {
        ui.add_space(15.0);
        let btn = Button::new(
            RichText::new(name)
                .size(20.0)
                .text_style(egui::TextStyle::Heading)
                .color(hex_color!("#ffffff")),
        )
        .min_size(egui::vec2(200.0, 0.0));
        btn
    }
}
