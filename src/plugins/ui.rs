use bevy_egui::egui::{Id, Order};

use super::*;

pub fn light_gray() -> Color32 {
    hex_color!("#6F6F6F")
}
pub fn dark_gray() -> Color32 {
    hex_color!("#393939")
}
pub fn black() -> Color32 {
    hex_color!("#000000")
}
pub fn light_black() -> Color32 {
    hex_color!("#202020")
}
pub fn white() -> Color32 {
    hex_color!("#ffffff")
}
pub fn yellow() -> Color32 {
    hex_color!("#D98F00")
}
pub fn red() -> Color32 {
    hex_color!("#E53935")
}
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup);
    }
}

impl UiPlugin {
    fn setup(world: &mut World) {
        let ctx = egui_context(world);
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "regular".to_owned(),
            FontData::from_static(include_bytes!(
                "../../assets/fonts/SometypeMono-Regular.ttf"
            )),
        );
        fonts.font_data.insert(
            "medium".to_owned(),
            FontData::from_static(include_bytes!("../../assets/fonts/SometypeMono-Medium.ttf")),
        );
        fonts.font_data.insert(
            "bold".to_owned(),
            FontData::from_static(include_bytes!("../../assets/fonts/SometypeMono-Bold.ttf")),
        );
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .insert(0, "regular".to_owned());
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "regular".to_owned());
        fonts
            .families
            .insert(FontFamily::Name("medium".into()), vec!["medium".to_owned()]);
        fonts
            .families
            .insert(FontFamily::Name("bold".into()), vec!["bold".to_owned()]);
        ctx.set_fonts(fonts);

        ctx.style_mut(|style| {
            style.text_styles = [
                (
                    TextStyle::Heading,
                    FontId::new(26.0, FontFamily::Name("medium".into())),
                ),
                (
                    TextStyle::Name("Heading2".into()),
                    FontId::new(22.0, FontFamily::Monospace),
                ),
                (
                    TextStyle::Name("Context".into()),
                    FontId::new(20.0, FontFamily::Monospace),
                ),
                (TextStyle::Body, FontId::new(14.0, FontFamily::Monospace)),
                (
                    TextStyle::Monospace,
                    FontId::new(14.0, FontFamily::Monospace),
                ),
                (TextStyle::Button, FontId::new(16.0, FontFamily::Monospace)),
                (TextStyle::Small, FontId::new(10.0, FontFamily::Monospace)),
            ]
            .into();
            style.visuals.window_fill = black();
            style.visuals.window_stroke.color = white();
            style.visuals.window_stroke.width = 1.0;
            // style.spacing.window_margin = 8.0.into();
            style.spacing.window_margin = 0.0.into();
            // dbg!(style.visuals.widgets.hovered);
            style.visuals.widgets.inactive = WidgetVisuals {
                bg_fill: white(),
                weak_bg_fill: black(),
                bg_stroke: Stroke::new(1.0, white()),
                rounding: Rounding::same(0.0),
                fg_stroke: Stroke::new(1.0, white()),
                expansion: 0.0,
            };
            style.visuals.widgets.active = WidgetVisuals {
                bg_fill: yellow(),
                weak_bg_fill: yellow(),
                bg_stroke: Stroke::NONE,
                rounding: Rounding::same(0.0),
                fg_stroke: Stroke::new(1.0, white()),
                expansion: 0.0,
            };
            style.visuals.widgets.hovered = WidgetVisuals {
                bg_fill: white(),
                weak_bg_fill: black(),
                bg_stroke: Stroke::new(1.0, yellow()),
                rounding: Rounding::same(0.0),
                fg_stroke: Stroke::new(1.0, yellow()),
                expansion: 1.0,
            };
            style.visuals.widgets.noninteractive = WidgetVisuals {
                bg_fill: black(),
                weak_bg_fill: black(),
                bg_stroke: Stroke::NONE,
                rounding: Rounding::same(0.0),
                fg_stroke: Stroke::new(1.0, light_gray()),
                expansion: 0.0,
            };
            // style.spacing.item_spacing = [12.0, 2.0].into();
        });
    }
}

pub fn text_dots_text(text1: &ColoredString, text2: &ColoredString, ui: &mut Ui) {
    ui.horizontal(|ui| {
        let rect = ui.max_rect();
        let left = rect.left() + ui.add(Label::new(text1.widget())).rect.width() + 3.0;
        let right = rect.right()
            - 3.0
            - ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                ui.label(text2.widget());
            })
            .response
            .rect
            .width();
        let bottom = rect.bottom() - 6.0;
        let line = egui::Shape::dotted_line(
            &[[left, bottom].into(), [right, bottom].into()],
            light_gray(),
            8.0,
            0.5,
        );
        ui.painter().add(line);
    });
}

pub struct GameWindow<'a> {
    area: Area,
    frame: Option<Frame>,
    title: &'a str,
    title_bar: bool,
    stroke: bool,
    width: f32,
    color: Option<Color32>,
}

impl GameWindow<'_> {
    pub fn show(self, ctx: &egui::Context, add_contents: impl FnOnce(&mut Ui)) {
        self.area.show(ctx, |ui| {
            self.show_ui(ui, add_contents);
        });
    }
    pub fn show_ui(mut self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
        if !self.stroke {
            self.frame = Some(Frame::none());
        }
        let style = ui.style();
        let mut stroke = style.visuals.window_stroke;
        stroke.color = self.color.unwrap_or(style.visuals.window_stroke.color);
        self.frame
            .unwrap_or(Frame::window(style).stroke(stroke))
            .show(ui, |ui| {
                ui.set_width(self.width);
                if self.title_bar {
                    let v = &ui.style().visuals.clone();
                    let mut rounding = v.window_rounding;
                    rounding.se = 0.0;
                    rounding.sw = 0.0;
                    Frame::none()
                        .fill(stroke.color)
                        .rounding(rounding)
                        .stroke(stroke)
                        .show(ui, |ui| {
                            ui.with_layout(
                                Layout::top_down(egui::Align::Min).with_cross_justify(true),
                                |ui| {
                                    Frame::none()
                                        .inner_margin(Margin::symmetric(8.0, 0.0))
                                        .show(ui, |ui| {
                                            ui.label(
                                                RichText::new(self.title)
                                                    .text_style(TextStyle::Heading)
                                                    .size(15.0)
                                                    .color(black()),
                                            );
                                        })
                                },
                            );
                        });
                }
                add_contents(ui)
            });
    }
    pub fn default_pos_vec(mut self, pos: Vec2) -> Self {
        self.area = self.area.default_pos(pos2(pos.x, pos.y));
        self
    }
    pub fn default_pos(mut self, pos: Pos2) -> Self {
        self.area = self.area.default_pos(pos);
        self
    }
    pub fn fixed_pos(mut self, pos: Vec2) -> Self {
        self.area = self.area.fixed_pos(pos2(pos.x, pos.y));
        self
    }
    pub fn id(mut self, id: impl std::hash::Hash) -> Self {
        self.area = self.area.id(Id::new(id).with(self.title));
        self
    }
    pub fn set_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
    pub fn anchor(mut self, align: Align2, offset: impl Into<egui::Vec2>) -> Self {
        self.area = self.area.anchor(align, offset);
        self
    }
    pub fn title_bar(mut self, enable: bool) -> Self {
        self.title_bar = enable;
        self
    }
    pub fn stroke(mut self, enable: bool) -> Self {
        self.stroke = enable;
        self
    }
    pub fn set_color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }
    pub fn order(mut self, order: Order) -> Self {
        self.area = self.area.order(order);
        self
    }
    pub fn entity_anchor(
        mut self,
        entity: Entity,
        pivot: Align2,
        offset: Vec2,
        world: &World,
    ) -> Self {
        let pos = entity_screen_pos(entity, offset, world);
        self.area = self.area.fixed_pos([pos.x, pos.y]).pivot(pivot);
        self
    }
}

pub fn window(title: &str) -> GameWindow<'_> {
    GameWindow {
        area: Area::new(title.to_owned())
            .constrain(true)
            .pivot(Align2::CENTER_CENTER),
        title,
        title_bar: true,
        stroke: true,
        color: None,
        width: 250.0,
        frame: None,
    }
}

pub fn frame<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    Frame::none()
        .stroke(Stroke::new(1.0, dark_gray()))
        .inner_margin(6.0)
        .outer_margin(6.0)
        .rounding(0.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.vertical_centered_justified(add_contents).inner
        })
}

pub fn frame_horizontal<R>(
    ui: &mut Ui,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    Frame::none()
        .stroke(Stroke::new(1.0, dark_gray()))
        .inner_margin(6.0)
        .outer_margin(6.0)
        .rounding(0.0)
        .show(ui, |ui| ui.horizontal_centered(add_contents).inner)
}

pub trait IntoC32 {
    fn c32(&self) -> Color32;
}

impl IntoC32 for Color {
    fn c32(&self) -> Color32 {
        let a = self.as_rgba_u8();
        Color32::from_rgba_unmultiplied(a[0], a[1], a[2], a[3])
    }
}

pub trait PrimarySecondaryExtensions {
    fn button_red(&mut self, text: impl Into<WidgetText>) -> Response;
    fn button_primary(&mut self, text: impl Into<WidgetText>) -> Response;
    fn button_or_primary(&mut self, text: impl Into<WidgetText>, primary: bool) -> Response;
}

impl PrimarySecondaryExtensions for Ui {
    fn button_primary(&mut self, text: impl Into<WidgetText>) -> Response {
        let style = self.style_mut();
        let prev_style = style.clone();
        style.visuals.widgets.inactive.weak_bg_fill = white();
        style.visuals.widgets.hovered.weak_bg_fill = white();
        style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, black());
        let response = Button::new(text).min_size(egui::Vec2::ZERO).ui(self);
        self.set_style(prev_style);
        response
    }

    fn button_or_primary(&mut self, text: impl Into<WidgetText>, primary: bool) -> Response {
        if primary {
            self.button_primary(text)
        } else {
            self.button(text)
        }
    }

    fn button_red(&mut self, text: impl Into<WidgetText>) -> Response {
        let visuals = &mut self.style_mut().visuals.widgets.inactive;
        visuals.fg_stroke.color = red();
        visuals.bg_stroke.color = red();
        self.button(text)
    }
}
