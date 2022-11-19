use super::*;
use std::collections::VecDeque;

mod field;
mod panel;
mod particle;
mod text;
mod unit;

use geng::{prelude::itertools::Itertools, Draw2d, PixelPerfectCamera};
use text::*;
pub use unit::*;

const DESCRIPTION_WIDTH: f32 = 2.0;
const DESCRIPTION_MARGIN: f32 = 0.1;
const FONT_SIZE: f32 = 0.2;

const DH_DESC_ARROW_SIZE: f32 = 0.1;
const DH_DESC_BACKGROUND: Rgba<f32> = Rgba {
    r: 0.2,
    g: 0.2,
    b: 0.2,
    a: 1.0,
};

const STATUS_DESC_ARROW_SIZE: f32 = 0.15;
const STATUS_DESC_FOREGROUND: Rgba<f32> = Rgba {
    r: 0.2,
    g: 0.2,
    b: 0.2,
    a: 1.0,
};
const STATUS_DESC_BACKGROUND: Rgba<f32> = Rgba {
    r: 0.1,
    g: 0.1,
    b: 0.1,
    a: 1.0,
};

#[derive(Clone)]
pub struct RenderModel {
    pub particles: Collection<Particle>,
    pub panels: Collection<Panel>,
    pub last_player_action_time: Time,
    pub last_enemy_action_time: Time,
    pub damage_instances: VecDeque<i32>,
    text_blocks: HashMap<Position, TextBlock>,
    texts: Vec<Text>,
}

#[derive(Debug, Clone)]
pub enum TextType {
    Damage(Vec<DamageType>),
    Status,
    Aoe,
    Message,
}

impl RenderModel {
    pub fn new() -> Self {
        Self {
            text_blocks: HashMap::new(),
            texts: Vec::new(),
            particles: Collection::new(),
            last_player_action_time: Time::ZERO,
            last_enemy_action_time: Time::ZERO,
            damage_instances: VecDeque::from(vec![1; 3]),
            panels: Collection::new(),
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        for text_block in self.text_blocks.values_mut() {
            text_block.update(delta_time);
        }
        for text in &mut self.texts {
            text.update(delta_time);
        }
        self.texts.retain(Text::is_alive);
    }
    pub fn clear(&mut self) {
        self.text_blocks.clear();
        self.texts.clear();
        self.particles.clear();
    }
    pub fn add_text(
        &mut self,
        position: Position,
        text: impl Into<String>,
        color: Rgba<f32>,
        text_type: TextType,
    ) {
        let text_block = self
            .text_blocks
            .entry(position)
            .or_insert_with(|| TextBlock::new(position.to_world_f32()));
        match text_type {
            TextType::Damage(_) => text_block.add_text_top(text, color, text_type),
            TextType::Status | TextType::Aoe => text_block.add_text_top(text, color, text_type),
            TextType::Message => text_block.add_text_top(text, color, text_type),
        }
    }
}

pub struct Render {
    geng: Geng,
    pub camera: geng::Camera2d,
    pub framebuffer_size: Vec2<f32>,
    assets: Rc<Assets>,
}

impl Render {
    pub fn new(geng: &Geng, assets: &Rc<Assets>, fov: f32) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            camera: geng::Camera2d {
                center: vec2(0.0, 0.0),
                rotation: 0.0,
                fov,
            },
            framebuffer_size: Vec2::ZERO,
        }
    }
    pub fn draw(&mut self, game_time: f32, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::WHITE), None, None);
        self.draw_field(
            &self.assets.custom_renders.field,
            game_time,
            model,
            framebuffer,
        );
        self.framebuffer_size = framebuffer.size().map(|x| x as f32);
        let mouse_world_pos = self.camera.screen_to_world(
            self.framebuffer_size,
            self.geng.window().mouse_pos().map(|x| x as f32),
        );

        let mut hovered_unit = None;

        for unit in &model.units {
            let template = &self.assets.units[&unit.unit_type];
            self.draw_unit(unit, game_time, framebuffer);

            self.draw_unit_stats(unit, framebuffer);

            // On unit hover
            if (mouse_world_pos - unit.render.render_position.map(|x| x.as_f32())).len()
                < unit.render.radius.as_f32()
            {
                // Draw extra ui: statuses descriptions, damage/heal descriptions
                hovered_unit = Some(unit);

                if let Some(unit) = hovered_unit {
                    let context = EffectContext {
                        owner: unit.id,
                        creator: unit.id,
                        target: unit.id,
                        vars: hashmap! {},
                        color: Rgba::BLACK,
                        queue_id: None,
                        status_id: None,
                    };
                    let vars: HashMap<VarName, i32> = unit
                        .template
                        .vars
                        .clone()
                        .into_iter()
                        .map(|(name, expr)| (name, expr.calculate(&context, model)))
                        .collect();
                    self.draw_hover(model, &unit, &self.camera, framebuffer, vars);
                }
            }
        }

        // Draw slots
        let factions = vec![Faction::Player, Faction::Enemy];
        let shader_program = &self.assets.custom_renders.slot;
        for faction in factions {
            for i in 0..SIDE_SLOTS {
                let quad = shader_program.get_vertices(&self.geng);
                let framebuffer_size = framebuffer.size();
                let position = Position {
                    x: i as i64,
                    side: faction,
                }
                .to_world_f32();
                let unit = model
                    .units
                    .iter()
                    .find(|unit| unit.position.x == i as i64 && unit.faction == faction);

                let health = match unit {
                    Some(unit) => 1.0,
                    None => 0.0,
                };

                ugli::draw(
                    framebuffer,
                    &shader_program.program,
                    ugli::DrawMode::TriangleStrip,
                    &quad,
                    (
                        ugli::uniforms! {
                            u_time: game_time,
                            u_unit_position: position,
                            u_parent_faction: match faction {
                                Faction::Player => 1.0,
                                Faction::Enemy => -1.0,
                            },
                            u_health: health,
                        },
                        geng::camera2d_uniforms(&self.camera, framebuffer_size.map(|x| x as f32)),
                        &shader_program.parameters,
                    ),
                    ugli::DrawParameters {
                        blend_mode: Some(ugli::BlendMode::default()),
                        ..default()
                    },
                );
            }
        }

        for particle in &model.render_model.particles {
            if particle.delay <= Time::new(0.0) {
                let render = self.assets.get_render(&particle.render_config); // TODO: move this into to an earlier phase perhaps
                self.draw_particle(particle, &render, game_time, framebuffer);
            }
        }

        for text in model
            .render_model
            .text_blocks
            .values()
            .flat_map(|text_block| text_block.texts())
        {
            let color = match &text.text_type {
                TextType::Damage(damage_types) => damage_types
                    .iter()
                    .filter_map(|damage_type| self.assets.damage_types.get(damage_type))
                    .sorted_by(|a, b| a.order.partial_cmp(&b.order).unwrap())
                    .find_map(|config| {
                        Some(config.color.unwrap_or_else(|| {
                            self.assets
                                .options
                                .clan_configs
                                .get(&config.clan_origin)
                                .unwrap_or_else(|| {
                                    panic!("Failed to find clan ({}) color", config.clan_origin)
                                })
                                .color
                        }))
                    })
                    .unwrap_or(text.color),
                _ => text.color,
            };
            self.geng.draw_2d(
                framebuffer,
                &self.camera,
                &draw_2d::Text::unit(&**self.geng.default_font(), &text.text, color)
                    .scale_uniform(0.15 * text.scale * text.size * text.scale)
                    .translate(text.render_position),
            );
        }

        if let Some(unit) = hovered_unit {
            self.draw_statuses_desc(unit, framebuffer);
            //self.unit_render.draw_hover(unit, &self.camera, framebuffer);
            if let Some(text_block) = model.render_model.text_blocks.get(&unit.position) {
                self.draw_damage_heal_desc(text_block, framebuffer);
            }
        }

        // Speed indicators
        let tick_text = vec!["x1", "x2", "x4"];
        let box_size = 0.15;

        for (i, text) in tick_text.into_iter().enumerate() {
            let is_active = model.time_modifier == 2.0 && i == 1
                || model.time_modifier == 4.0 && i == 2
                || model.time_modifier < 2.0 && i == 0;
            let mut text_scale = 1.0;
            let mut text_color = Rgba::try_from("#cccccc").unwrap();
            if is_active {
                text_scale = 1.3;
                text_color = Rgba::try_from("#4a4a4a").unwrap();
            }
            self.geng.draw_2d(
                framebuffer,
                &self.camera,
                &draw_2d::Text::unit(&**self.geng.default_font(), &text, text_color)
                    .scale_uniform(box_size * text_scale)
                    .translate(vec2(
                        (i as f32 - 1.0) * box_size * 4.0,
                        -self.camera.fov * 0.45,
                    )),
            );
        }

        if let Some(unit) = hovered_unit {
            self.draw_statuses_desc(unit, framebuffer);
            if let Some(text_block) = model.render_model.text_blocks.get(&unit.position) {
                self.draw_damage_heal_desc(text_block, framebuffer);
            }
        }

        for panel in &model.render_model.panels {
            self.draw_panel(panel, game_time, framebuffer);
        }
    }

    fn draw_statuses_desc(&self, unit: &Unit, framebuffer: &mut ugli::Framebuffer) {
        let font_size = FONT_SIZE;
        let mut statuses = HashMap::new();
        for status in &unit.all_statuses {
            *statuses.entry(status.status.name.clone()).or_insert(0) += 1;
        }
        let descriptions: Vec<_> = unit
            .all_statuses
            .iter()
            .filter_map(|status| {
                let status = status.status.name.clone();
                statuses.remove(&status).and_then(|stacks| {
                    self.assets
                        .statuses
                        .get(&status)
                        .filter(|config| !config.status.hidden)
                        .map(|config| {
                            let lines = wrap_text(
                                self.geng.default_font().clone(),
                                &config.description,
                                font_size,
                                DESCRIPTION_WIDTH,
                            )
                            .expect("Failed to measure text");
                            let height = (lines.len() as f32 + 1.5) * font_size;
                            (status, stacks, config, lines, height)
                        })
                })
            })
            .collect();
        if descriptions.is_empty() {
            return;
        }
        let total_height = descriptions.iter().map(|(_, _, _, _, h)| *h).sum::<f32>()
            + (descriptions.len() + 1) as f32 * DESCRIPTION_MARGIN;
        let top_left = vec2(
            unit.render.render_position.x.as_f32()
                + unit.render.radius.as_f32() / 2.0
                + DESCRIPTION_MARGIN
                + STATUS_DESC_ARROW_SIZE,
            unit.render.render_position.y.as_f32() + total_height / 2.0,
        );
        let bottom_right =
            top_left + vec2(DESCRIPTION_WIDTH + DESCRIPTION_MARGIN * 2.0, -total_height);

        draw_2d::Quad::new(
            AABB::from_corners(top_left, bottom_right),
            STATUS_DESC_BACKGROUND,
        )
        .draw_2d(&self.geng, framebuffer, &self.camera);
        let left_mid = vec2(top_left.x, unit.render.render_position.y.as_f32());
        draw_2d::Polygon::new(
            vec![
                left_mid - vec2(STATUS_DESC_ARROW_SIZE, 0.0),
                left_mid - vec2(0.0, STATUS_DESC_ARROW_SIZE),
                left_mid + vec2(0.0, STATUS_DESC_ARROW_SIZE),
            ],
            STATUS_DESC_BACKGROUND,
        )
        .draw_2d(&self.geng, framebuffer, &self.camera);

        let mut text_pos = top_left
            + vec2(
                DESCRIPTION_MARGIN + DESCRIPTION_WIDTH / 2.0,
                -DESCRIPTION_MARGIN - font_size,
            );
        for (mut status, stacks, config, description, height) in descriptions {
            draw_2d::Quad::new(
                AABB::point(text_pos)
                    .extend_symmetric(vec2(DESCRIPTION_WIDTH / 2.0, 0.0))
                    .extend_up(font_size)
                    .extend_down(height - font_size),
                STATUS_DESC_FOREGROUND,
            )
            .draw_2d(&self.geng, framebuffer, &self.camera);

            let color = config.get_color(&self.assets.options);
            let font = self.geng.default_font().clone();
            if stacks > 1 {
                status.push_str(&format!(" ({stacks})"));
            }
            draw_text(
                font.clone(),
                framebuffer,
                &self.camera,
                &status,
                text_pos,
                geng::TextAlign::CENTER,
                font_size,
                color,
            );
            draw_lines(
                font,
                &description,
                font_size,
                text_pos,
                Rgba::try_from("#6d6d6d").unwrap(),
                framebuffer,
                &self.camera,
            );

            text_pos.y -= height + DESCRIPTION_MARGIN;
        }
    }

    fn draw_damage_heal_desc(&self, text_block: &TextBlock, framebuffer: &mut ugli::Framebuffer) {
        /// Converts texts into descriptions
        type DHRow<'a> = Vec<(&'a String, &'a DamageHealConfig)>;
        fn to_descriptions<'a>(
            assets: &'a Rc<Assets>,
            texts: impl IntoIterator<Item = &'a Text>,
        ) -> Vec<(DHRow<'a>, Vec2<f32>)> {
            texts
                .into_iter()
                .filter_map(|text| match &text.text_type {
                    TextType::Damage(damage_types) => Some((
                        damage_types
                            .iter()
                            .filter_map(|damage_type| {
                                assets
                                    .damage_types
                                    .get(damage_type)
                                    .map(|config| (damage_type, config))
                            })
                            .sorted_by(|a, b| a.1.order.partial_cmp(&b.1.order).unwrap())
                            .collect(),
                        text.position,
                    )),
                    TextType::Status => None,
                    TextType::Aoe => None,
                    TextType::Message => None,
                })
                .collect()
        }

        /// Layout and render descriptions
        fn draw_descriptions<'a>(
            mut descriptions: Vec<(DHRow<'a>, Vec2<f32>)>,
            y_scale: f32,
            assets: &Rc<Assets>,
            geng: &Geng,
            framebuffer: &mut ugli::Framebuffer,
            camera: &impl geng::AbstractCamera2d,
        ) {
            descriptions.sort_by_key(|(_, pos)| r32(pos.y * y_scale));
            let mut last_aabb: Option<AABB<f32>> = None;

            for (desc, pos) in descriptions
                .into_iter()
                .filter(|(desc, _)| !desc.is_empty())
            {
                let font = geng.default_font().clone();

                let (offset, extra_space) = last_aabb
                    .map(|aabb| {
                        let delta = if y_scale > 0.0 {
                            pos.y - aabb.y_max
                        } else {
                            aabb.y_min - pos.y
                        };
                        if delta < 0.0 {
                            (-aabb.width(), None)
                        } else {
                            (0.0, Some(delta))
                        }
                    })
                    .unwrap_or((0.0, None));

                fn aabb_union<T: UNum>(a: &AABB<T>, b: &AABB<T>) -> AABB<T> {
                    AABB::points_bounding_box(a.corners().into_iter().chain(b.corners()))
                }

                let pos = pos + vec2(offset - DESCRIPTION_MARGIN - DH_DESC_ARROW_SIZE, 0.0);
                let mut desc_aabb = AABB::point(pos);
                draw_2d::Polygon::new(
                    vec![
                        pos + vec2(0.0, DH_DESC_ARROW_SIZE),
                        pos + vec2(DH_DESC_ARROW_SIZE, 0.0),
                        pos + vec2(0.0, -DH_DESC_ARROW_SIZE),
                    ],
                    DH_DESC_BACKGROUND,
                )
                .draw_2d(geng, framebuffer, camera);

                for (name, config) in desc {
                    let width = DESCRIPTION_WIDTH;
                    let font_size = FONT_SIZE;
                    let lines = wrap_text(font.clone(), &config.description, font_size, width)
                        .expect("Failed to wrap text");
                    let height = (lines.len() as f32 + 1.5) * font_size;
                    let space = match extra_space {
                        None => height / 2.0,
                        Some(space) => space.min(height / 2.0),
                    };
                    let aabb = AABB::point(vec2(desc_aabb.x_min, desc_aabb.center().y))
                        .extend_up(if y_scale > 0.0 { height - space } else { space })
                        .extend_down(if y_scale > 0.0 { space } else { height - space })
                        .extend_left(width);
                    desc_aabb = aabb_union(&desc_aabb, &aabb).extend_left(DESCRIPTION_MARGIN);

                    draw_2d::Quad::new(aabb, DH_DESC_BACKGROUND).draw_2d(geng, framebuffer, camera);

                    let color = config.color.unwrap_or_else(|| {
                        assets
                            .options
                            .clan_configs
                            .get(&config.clan_origin)
                            .unwrap_or_else(|| {
                                panic!("Failed to find clan ({}) color", config.clan_origin)
                            })
                            .color
                    });
                    let pos = vec2(aabb.center().x, aabb.y_max - font_size);
                    draw_text(
                        font.clone(),
                        framebuffer,
                        camera,
                        name,
                        pos,
                        geng::TextAlign::CENTER,
                        font_size,
                        color,
                    );
                    draw_lines(
                        font.clone(),
                        &lines,
                        font_size,
                        pos,
                        Rgba::try_from("#6d6d6d").unwrap(),
                        framebuffer,
                        camera,
                    );
                }

                last_aabb = Some(desc_aabb);
            }
        }

        draw_descriptions(
            to_descriptions(&self.assets, text_block.top_texts()),
            1.0,
            &self.assets,
            &self.geng,
            framebuffer,
            &self.camera,
        );
        draw_descriptions(
            to_descriptions(&self.assets, text_block.bot_texts()),
            -1.0,
            &self.assets,
            &self.geng,
            framebuffer,
            &self.camera,
        );
    }
}

pub fn wrap_text(
    font: impl std::borrow::Borrow<geng::Font>,
    text: impl AsRef<str>,
    font_size: f32,
    target_width: f32,
) -> Option<Vec<String>> {
    let font = font.borrow();
    let text = text.as_ref();

    let measure = |text| {
        const SIZE_HACK: f32 = 1000.0;
        font.measure(text, SIZE_HACK)
            .map(|aabb| aabb.width() / SIZE_HACK * font_size)
    };

    let space_width = measure("_ _")? - measure("__")?;

    let mut lines = Vec::new();
    for line in text.lines() {
        let mut words = line.split_whitespace();
        let mut line = String::new();
        let mut line_width = 0.0;
        if let Some(word) = words.next() {
            let width = measure(word)?;
            line_width += width;
            line += word;
        }
        for word in words {
            let width = measure(word)?;
            if line_width + space_width + width <= target_width {
                line_width += space_width + width;
                line += " ";
                line += word;
            } else {
                lines.push(line);
                line = word.to_owned();
                line_width = width;
                continue;
            }
        }
        lines.push(line);
    }
    Some(lines)
}

/// Hacks the limitation in small font sizes to accurately align text
#[allow(clippy::too_many_arguments)]
pub fn draw_text(
    font: impl std::borrow::Borrow<geng::Font>,
    framebuffer: &mut ugli::Framebuffer,
    camera: &impl geng::AbstractCamera2d,
    text: impl AsRef<str>,
    position: Vec2<f32>,
    text_align: geng::TextAlign,
    font_size: f32,
    color: Rgba<f32>,
) {
    const SIZE_HACK: f32 = 1000.0;
    let font = font.borrow();
    let text = text.as_ref();

    let offset = font
        .measure(text, SIZE_HACK)
        .expect("Failed to measure text")
        .width()
        / SIZE_HACK
        * font_size
        * text_align.0;
    font.draw(
        framebuffer,
        camera,
        text,
        position - vec2(offset, 0.0),
        geng::TextAlign::LEFT,
        font_size,
        color,
    );
}

pub fn draw_lines(
    font: impl std::borrow::Borrow<geng::Font>,
    lines: &[impl AsRef<str>],
    font_size: f32,
    top_anchor: Vec2<f32>,
    color: Rgba<f32>,
    framebuffer: &mut ugli::Framebuffer,
    camera: &impl geng::AbstractCamera2d,
) {
    let font = font.borrow();
    let mut pos = vec2(top_anchor.x, top_anchor.y - font_size);
    for line in lines {
        const SIZE_HACK: f32 = 1000.0;
        draw_text(
            font,
            framebuffer,
            camera,
            line,
            pos,
            geng::TextAlign::CENTER,
            font_size,
            color,
        );
        pos.y -= font_size;
    }
}

pub fn draw_text_wrapped(
    font: impl std::borrow::Borrow<geng::Font>,
    text: impl AsRef<str>,
    font_size: f32,
    target: AABB<f32>,
    color: Rgba<f32>,
    framebuffer: &mut ugli::Framebuffer,
    camera: &impl geng::AbstractCamera2d,
) -> Option<()> {
    let font = font.borrow();
    let lines = wrap_text(font, text, font_size, target.width())?;
    draw_lines(
        font,
        &lines,
        font_size,
        vec2(target.center().x, target.y_max),
        color,
        framebuffer,
        camera,
    );
    Some(())
}
