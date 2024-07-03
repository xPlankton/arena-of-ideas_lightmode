use egui::{FontId, TextStyle};

use super::*;

#[macro_export]
macro_rules! hex_color_noa {
    ($s:literal) => {{
        let array = color_hex::color_from_hex!($s);
        $crate::Color32::from_rgb(array[0], array[1], array[2])
    }};
}

pub const WHITE: Color32 = hex_color_noa!("#FFFFFF");
pub const DARK_WHITE: Color32 = hex_color_noa!("#BFBFBF");
pub const LIGHT_GRAY: Color32 = hex_color_noa!("#6F6F6F");
pub const GRAY: Color32 = hex_color_noa!("#4F4F4F");
pub const DARK_GRAY: Color32 = hex_color_noa!("#373737");
pub const LIGHT_BLACK: Color32 = hex_color_noa!("#202020");
pub const DARK_BLACK: Color32 = hex_color_noa!("#131313");
pub const BLACK: Color32 = hex_color_noa!("#000000");

pub const YELLOW: Color32 = hex_color_noa!("#D98F00");
pub const RED: Color32 = hex_color_noa!("#E53935");
pub const GREEN: Color32 = hex_color_noa!("#64DD17");

pub const TRANSPARENT: Color32 = Color32::TRANSPARENT;

pub const SHADOW: Shadow = Shadow {
    offset: egui::vec2(10.0, 20.0),
    blur: 15.0,
    spread: 0.0,
    color: Color32::from_black_alpha(96),
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup)
            .init_resource::<WidgetData>();
    }
}

#[derive(Resource, Default, Debug)]
pub struct WidgetData {
    pub unit_container: HashMap<Faction, UnitContainerData>,
    pub notifications: NotificationsData,
}

impl UiPlugin {
    fn setup(world: &mut World) {
        let Some(ctx) = egui_context(world) else {
            return;
        };
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
                    TextStyle::Name("Bold".into()),
                    FontId::new(14.0, FontFamily::Name("bold".into())),
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
            style.spacing = Spacing {
                item_spacing: egui::vec2(13.0, 6.0),
                button_padding: egui::vec2(5.0, 6.0),
                ..default()
            };
            style.spacing.slider_rail_height = 2.0;
            style.visuals.slider_trailing_fill = true;
            style.visuals.handle_shape = HandleShape::Rect { aspect_ratio: 0.1 };
            style.visuals.selection.bg_fill = WHITE;
            style.visuals.resize_corner_size = 0.0;
            style.visuals.window_stroke = Stroke {
                width: 1.0,
                color: DARK_GRAY,
            };
            style.spacing.window_margin = Margin::same(13.0);
            style.visuals.window_fill = LIGHT_BLACK;
            style.visuals.widgets = Widgets {
                noninteractive: WidgetVisuals {
                    weak_bg_fill: Color32::TRANSPARENT,
                    bg_fill: Color32::from_gray(27),
                    bg_stroke: Stroke::new(1.0, GRAY), // separators, indentation lines
                    fg_stroke: Stroke::new(1.0, LIGHT_GRAY), // normal text color
                    rounding: Rounding::same(13.0),
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    weak_bg_fill: Color32::TRANSPARENT,
                    bg_fill: LIGHT_GRAY, // checkbox background
                    bg_stroke: Default::default(),
                    fg_stroke: Stroke::new(1.0, WHITE), // button text
                    rounding: Rounding::same(13.0),
                    expansion: 0.0,
                },
                hovered: WidgetVisuals {
                    weak_bg_fill: Color32::TRANSPARENT,
                    bg_fill: Color32::from_gray(70),
                    bg_stroke: Stroke::new(1.0, LIGHT_GRAY), // e.g. hover over window edge or button
                    fg_stroke: Stroke::new(1.5, WHITE),
                    rounding: Rounding::same(13.0),
                    expansion: 1.0,
                },
                active: WidgetVisuals {
                    weak_bg_fill: Color32::TRANSPARENT,
                    bg_fill: Color32::from_gray(55),
                    bg_stroke: Stroke::new(1.0, YELLOW),
                    fg_stroke: Stroke::new(2.0, YELLOW),
                    rounding: Rounding::same(13.0),
                    expansion: 1.0,
                },
                open: WidgetVisuals {
                    weak_bg_fill: Color32::from_gray(45),
                    bg_fill: Color32::from_gray(27),
                    bg_stroke: Stroke::new(1.0, Color32::from_gray(60)),
                    fg_stroke: Stroke::new(1.0, Color32::from_gray(210)),
                    rounding: Rounding::same(13.0),
                    expansion: 0.0,
                },
            };
        });
    }
}

const PATH: &str = "tile_path";
pub trait CtxExt {
    fn is_name_enabled(&self, name: &str) -> bool;
    fn is_path_enabled(&self, path: &str) -> bool;
    fn cur_enabled(&self) -> bool;
    fn flip_name_enabled(&self, name: &str);
    fn flip_path_enabled(&self, path: &str);
    fn path(&self) -> String;
    fn path_with(&self, name: &str) -> String;
    fn add_path(&self, name: &str);
    fn remove_path(&self);
}

impl CtxExt for egui::Context {
    fn is_name_enabled(&self, name: &str) -> bool {
        self.data(|r| r.get_temp(self.path_with(name).into()))
            .unwrap_or_default()
    }
    fn is_path_enabled(&self, path: &str) -> bool {
        self.data(|r| r.get_temp(Id::new(path))).unwrap_or_default()
    }
    fn cur_enabled(&self) -> bool {
        self.is_path_enabled(&self.path())
    }
    fn flip_name_enabled(&self, name: &str) {
        let p = self.path_with(name);
        self.flip_path_enabled(&p)
    }
    fn flip_path_enabled(&self, path: &str) {
        let v = self.is_path_enabled(&path);
        self.data_mut(|w| w.insert_temp(Id::new(path), !v))
    }
    fn path(&self) -> String {
        self.data(|r| r.get_temp(Id::new(PATH))).unwrap_or_default()
    }
    fn path_with(&self, name: &str) -> String {
        let p = self.path();
        format!("{p}/{name}")
    }
    fn add_path(&self, name: &str) {
        let mut p = self.path();
        p.push('/');
        p.push_str(name);
        self.data_mut(|w| w.insert_temp(Id::new(PATH), p));
    }
    fn remove_path(&self) {
        let mut p = self.path();
        if let Some(pos) = p.rfind('/') {
            let _ = p.split_off(pos);
            self.data_mut(|w| w.insert_temp(Id::new(PATH), p));
        }
    }
}

#[derive(Default, Debug)]
pub enum Side {
    #[default]
    Right,
    Left,
    Top,
    Bottom,
}
