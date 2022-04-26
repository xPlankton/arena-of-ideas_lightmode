use super::*;

#[derive(Clone)]
struct Text {
    position: Vec2<f32>,
    velocity: Vec2<f32>,
    time: f32,
    text: String,
    color: Color<f32>,
}

#[derive(Clone)]
pub struct RenderModel {
    texts: Vec<Text>,
}

impl RenderModel {
    pub fn new() -> Self {
        Self { texts: Vec::new() }
    }
    pub fn update(&mut self, delta_time: f32) {
        for text in &mut self.texts {
            text.time += delta_time;
            text.position += text.velocity * delta_time;
        }
        self.texts.retain(|text| text.time < 1.0);
    }
    pub fn add_text(&mut self, position: Vec2<Coord>, text: &str, color: Color<f32>) {
        let velocity = vec2(0.2, 0.0).rotate(global_rng().gen_range(0.0..2.0 * f32::PI));
        self.texts.push(Text {
            position: position.map(|x| x.as_f32()) + velocity,
            time: 0.0,
            velocity,
            text: text.to_owned(),
            color,
        });
    }
}

pub struct Render {
    geng: Geng,
    camera: geng::Camera2d,
    assets: Assets,
}

impl Render {
    pub fn new(geng: &Geng, assets: Assets, config: Config) -> Self {
        Self {
            geng: geng.clone(),
            assets,
            camera: geng::Camera2d {
                center: vec2(0.0, 0.0),
                rotation: 0.0,
                fov: config.fov,
            },
        }
    }
    pub fn draw(
        &mut self,
        game_time: f32,
        model: &Model,
        render_model: &RenderModel,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);
        for unit in itertools::chain![&model.units, &model.spawning_units] {
            let template = &self.assets.units[&unit.unit_type];

            let render = self.assets.get_render(&unit.render); // TODO: move this into to an earlier phase perhaps
            self.draw_unit(unit, template, &render, model, game_time, framebuffer);
            if unit
                .all_statuses
                .iter()
                .any(|status| status.r#type() == StatusType::Shield)
            {
                self.geng.draw_2d(
                    framebuffer,
                    &self.camera,
                    &draw_2d::Ellipse::circle(
                        unit.position.map(|x| x.as_f32()),
                        unit.radius.as_f32() * 1.1,
                        Color::rgba(1.0, 1.0, 0.0, 0.5),
                    ),
                );
            }
            self.geng.draw_2d(
                framebuffer,
                &self.camera,
                &draw_2d::Quad::unit(Color::GREEN).transform(
                    Mat3::translate(unit.position.map(|x| x.as_f32()))
                        * Mat3::scale_uniform(unit.radius.as_f32())
                        * Mat3::translate(vec2(0.0, 1.2))
                        * Mat3::scale(
                            0.1 * vec2(10.0 * unit.health.as_f32() / unit.max_hp.as_f32(), 1.0),
                        ),
                ),
            );
        }
        for projectile in &model.projectiles {
            self.geng.draw_2d(
                framebuffer,
                &self.camera,
                &draw_2d::Ellipse::circle(projectile.position.map(|x| x.as_f32()), 0.1, Color::RED),
            );
        }
        for particle in &model.particles {
            let render = self.assets.get_render(&particle.render_config); // TODO: move this into to an earlier phase perhaps
            self.draw_particle(particle, &render, game_time, framebuffer);
        }
        for text in &render_model.texts {
            self.geng.draw_2d(
                framebuffer,
                &self.camera,
                &draw_2d::Text::unit(&**self.geng.default_font(), &text.text, text.color)
                    .scale_uniform(0.05)
                    .translate(text.position),
            );
        }
    }

    fn draw_unit(
        &self,
        unit: &Unit,
        template: &UnitTemplate,
        render_mode: &RenderMode,
        model: &Model,
        game_time: f32,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        let spawn_scale = match unit.spawn_animation_time_left {
            Some(time) if template.spawn_animation_time > Time::new(0.0) => {
                1.0 - (time / template.spawn_animation_time).as_f32()
            }
            _ => 1.0,
        };
        let attack_scale = match &unit.action_state {
            ActionState::Start { time, .. } => {
                1.0 - 0.25 * (*time / unit.action.animation_delay).as_f32()
            }
            _ => 1.0,
        };

        match render_mode {
            RenderMode::Circle { color } => {
                self.geng.draw_2d(
                    framebuffer,
                    &self.camera,
                    &draw_2d::Ellipse::circle(
                        unit.position.map(|x| x.as_f32()),
                        unit.radius.as_f32() * attack_scale * spawn_scale,
                        {
                            let mut color = *color;
                            if unit
                                .all_statuses
                                .iter()
                                .any(|status| status.r#type() == StatusType::Freeze)
                            {
                                color = Color::CYAN;
                            }
                            if unit
                                .all_statuses
                                .iter()
                                .any(|status| matches!(status, Status::Slow { .. }))
                            {
                                color = Color::GRAY;
                            }
                            color
                        },
                    ),
                );
            }
            RenderMode::Texture { texture } => {
                self.geng.draw_2d(
                    framebuffer,
                    &self.camera,
                    &draw_2d::TexturedQuad::unit(&**texture)
                        .scale_uniform(unit.radius.as_f32() * attack_scale * spawn_scale)
                        .translate(unit.position.map(|x| x.as_f32())),
                );
            }
            RenderMode::Shader {
                program,
                parameters,
            } => {
                let quad = ugli::VertexBuffer::new_dynamic(
                    self.geng.ugli(),
                    vec![
                        draw_2d::Vertex {
                            a_pos: vec2(-1.0, -1.0),
                        },
                        draw_2d::Vertex {
                            a_pos: vec2(1.0, -1.0),
                        },
                        draw_2d::Vertex {
                            a_pos: vec2(1.0, 1.0),
                        },
                        draw_2d::Vertex {
                            a_pos: vec2(-1.0, 1.0),
                        },
                    ],
                );
                let framebuffer_size = framebuffer.size();
                let model_matrix = Mat3::translate(unit.position.map(|x| x.as_f32()))
                    * Mat3::scale_uniform(unit.radius.as_f32() * attack_scale * spawn_scale);

                let mut alliances: Vec<Alliance> = unit.alliances.iter().copied().collect();
                let alliance_colors: Vec<Color<f32>> = alliances
                    .iter()
                    .map(|alliance| self.assets.options.alliance_colors[alliance])
                    .collect();

                let (action_time, target) = match &unit.action_state {
                    ActionState::Start { time, target } => (
                        (*time / unit.action.animation_delay).as_f32(),
                        model.units.get(&target).map(|unit| unit),
                    ),
                    _ => (0.0, None),
                };

                let target_dir = target
                    .map_or(Vec2::ZERO, |target| target.position - unit.position)
                    .map(|x| x.as_f32());

                ugli::draw(
                    framebuffer,
                    program,
                    ugli::DrawMode::TriangleFan,
                    &quad,
                    (
                        ugli::uniforms! {
                            u_time: game_time,
                            u_unit_position: unit.position.map(|x| x.as_f32()),
                            u_unit_radius: unit.radius.as_f32(),
                            u_spawn: spawn_scale,
                            u_action: action_time,
                            u_cooldown: unit.action.cooldown.as_f32(),
                            u_target_dir: target_dir,
                            u_random: unit.random_number.as_f32(),
                            u_alliance_color_1: alliance_colors.get(0).copied().unwrap_or(Color::WHITE),
                            u_alliance_color_2: alliance_colors.get(1).copied().unwrap_or(Color::WHITE),
                            u_alliance_color_3: alliance_colors.get(2).copied().unwrap_or(Color::WHITE),
                            u_alliance_count: alliance_colors.len(),
                        },
                        geng::camera2d_uniforms(&self.camera, framebuffer_size.map(|x| x as f32)),
                        parameters,
                    ),
                    ugli::DrawParameters {
                        blend_mode: Some(default()),
                        ..default()
                    },
                );
            }
        }
    }

    fn draw_particle(
        &self,
        particle: &Particle,
        render_mode: &RenderMode,
        game_time: f32,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        match render_mode {
            RenderMode::Circle { color } => {
                self.geng.draw_2d(
                    framebuffer,
                    &self.camera,
                    &draw_2d::Ellipse::circle(
                        particle.position.map(|x| x.as_f32()),
                        particle.radius.as_f32(),
                        *color,
                    ),
                );
            }
            RenderMode::Texture { texture } => {
                self.geng.draw_2d(
                    framebuffer,
                    &self.camera,
                    &draw_2d::TexturedQuad::unit(&**texture)
                        .scale_uniform(particle.radius.as_f32())
                        .translate(particle.position.map(|x| x.as_f32())),
                );
            }
            RenderMode::Shader {
                program,
                parameters,
            } => {
                let quad = ugli::VertexBuffer::new_dynamic(
                    self.geng.ugli(),
                    vec![
                        draw_2d::Vertex {
                            a_pos: vec2(-1.0, -1.0),
                        },
                        draw_2d::Vertex {
                            a_pos: vec2(1.0, -1.0),
                        },
                        draw_2d::Vertex {
                            a_pos: vec2(1.0, 1.0),
                        },
                        draw_2d::Vertex {
                            a_pos: vec2(-1.0, 1.0),
                        },
                    ],
                );
                let framebuffer_size = framebuffer.size();
                let model_matrix = Mat3::translate(particle.position.map(|x| x.as_f32()))
                    * Mat3::scale_uniform(particle.radius.as_f32());

                ugli::draw(
                    framebuffer,
                    program,
                    ugli::DrawMode::TriangleFan,
                    &quad,
                    (
                        ugli::uniforms! {
                            u_time: game_time,
                            u_unit_position: particle.position.map(|x| x.as_f32()),
                            u_unit_radius: particle.radius.as_f32(),
                            u_spawn: (particle.time_left / particle.duration).as_f32(),
                            u_action: 0.0,
                            u_alliance_color_1: Color::WHITE,
                            u_alliance_color_2: Color::WHITE,
                            u_alliance_color_3: Color::WHITE,
                            u_alliance_count: 0,
                        },
                        geng::camera2d_uniforms(&self.camera, framebuffer_size.map(|x| x as f32)),
                        parameters,
                    ),
                    ugli::DrawParameters {
                        blend_mode: Some(default()),
                        ..default()
                    },
                );
            }
        }
    }
}
