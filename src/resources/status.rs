use geng::prelude::itertools::Itertools;

use super::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Status {
    pub trigger: Trigger,
    pub description: Option<String>,
    #[serde(default = "default_color")]
    pub color: Rgba<f32>,
    pub shader: Option<ShaderChain>,
}

fn default_color() -> Rgba<f32> {
    Rgba::MAGENTA
}

impl Status {
    pub fn notify_all(
        event: &Event,
        factions: &HashSet<Faction>,
        context: &Context,
        world: &legion::World,
        resources: &mut Resources,
    ) {
        let units = UnitSystem::collect_factions_states(world, factions);
        for (unit, _) in units
            .into_iter()
            .sorted_by_key(|(_, state)| state.get_int(&VarName::Slot, world))
        {
            Self::notify_one(event, unit, context, world, resources);
        }
    }

    pub fn notify_one(
        event: &Event,
        entity: legion::Entity,
        context: &Context,
        world: &legion::World,
        resources: &mut Resources,
    ) {
        let context = context.clone_stack(ContextLayer::Unit { entity }, world, resources);
        let statuses = context.collect_statuses(world);
        let mut triggers = StatusLibrary::add_triggers(statuses, resources);
        if let Ok(trigger) = world.entry_ref(entity).unwrap().get_component::<Trigger>() {
            triggers.push(("_local".to_owned(), (trigger.clone(), 1)));
        }
        for (name, (trigger, charges)) in triggers {
            let context = context.clone_stack(
                ContextLayer::Status {
                    entity,
                    name,
                    charges,
                },
                world,
                resources,
            );
            trigger.catch_event(
                event,
                &mut resources.action_queue,
                context,
                &resources.logger,
            );
        }
    }

    pub fn notify_status(
        event: &Event,
        entity: legion::Entity,
        context: &Context,
        name: &str,
        charges: i32,
        world: &legion::World,
        resources: &mut Resources,
    ) {
        let mut context = context.clone_stack(ContextLayer::Entity { entity }, world, resources);
        context.stack(
            ContextLayer::Status {
                entity,
                name: name.to_owned(),
                charges,
            },
            world,
            resources,
        );
        let trigger = StatusLibrary::get_trigger(name, resources);
        trigger.catch_event(
            event,
            &mut resources.action_queue,
            context,
            &resources.logger,
        );
    }

    pub fn calculate_one(
        event: Event,
        entity: legion::Entity,
        context: &mut Context,
        world: &legion::World,
        resources: &Resources,
    ) {
        let statuses = context.collect_statuses(world);
        let mut triggers = StatusLibrary::add_triggers(statuses, resources);
        if let Ok(trigger) = world.entry_ref(entity).unwrap().get_component::<Trigger>() {
            triggers.push(("_local".to_owned(), (trigger.clone(), 1)));
        }
        for (name, (trigger, charges)) in triggers {
            let status_context = context.clone_stack(
                ContextLayer::Status {
                    entity,
                    name,
                    charges,
                },
                world,
                resources,
            );
            match trigger.calculate_event(&event, &status_context, world, resources) {
                Ok(extra_layers) => {
                    for layer in extra_layers {
                        context.stack(layer, world, resources);
                    }
                }
                Err(error) => {
                    resources.logger.log(
                        || format!("Failed to calculate trigger {error}\n{trigger:?}"),
                        &LogContext::Trigger,
                    );
                }
            }
        }
    }

    pub fn change_charges(
        entity: legion::Entity,
        delta: i32,
        name: &str,
        node: &mut Option<Node>,
        world: &mut legion::World,
        resources: &mut Resources,
    ) {
        let before = Context::new(ContextLayer::Entity { entity }, world, resources)
            .get_status_charges(name, world);
        let after = before + delta;
        let state = ContextState::get_mut(entity, world);
        *state.statuses.entry(name.to_owned()).or_default() += delta;
        state.t += 1;
        *state.status_change_t.entry(name.to_owned()).or_default() = state.t;

        match delta.signum() {
            1 => Event::StatusChargeAdd {
                status: name.to_owned(),
                owner: entity,
                charges: after,
            }
            .send(world, resources),
            -1 => Event::StatusChargeRemove {
                status: name.to_owned(),
                owner: entity,
                charges: after,
            }
            .send(world, resources),
            _ => {}
        }
        for _ in 0..delta.abs() {
            let (text, color) = if delta > 0 {
                ("+", resources.options.colors.add)
            } else {
                ("-", resources.options.colors.subtract)
            };

            if let Some(node) = node.as_mut() {
                let text = format!("{}{}", text, &name);
                let outline_color = StatusLibrary::get(name, resources).color;
                VfxSystem::vfx_show_parent_text(
                    node,
                    &text,
                    color,
                    outline_color,
                    entity,
                    1,
                    resources,
                );
            }
        }
        if before <= 0 && after > 0 {
            Event::StatusAdd {
                status: name.to_owned(),
                owner: entity,
                charges: after,
            }
            .send(world, resources);
        } else if before > 0 && after <= 0 {
            Event::StatusRemove {
                status: name.to_owned(),
                owner: entity,
                charges: after,
            }
            .send(world, resources);
        }
    }

    pub fn clear_entity(entity: legion::Entity, world: &mut legion::World) {
        if let Some(mut entry) = world.entry(entity) {
            if let Ok(state) = entry.get_component_mut::<ContextState>() {
                state.statuses.clear();
            }
        }
    }

    pub fn generate_card_shader(
        &self,
        name: &str,
        charges: Option<i32>,
        resources: &Resources,
    ) -> ShaderChain {
        let mut shader = resources.options.shaders.unit_card.clone();
        if let Some(self_shader) = self.shader.as_ref() {
            shader.before.push(
                self_shader
                    .clone()
                    .insert_int("u_index".to_owned(), 0)
                    .insert_float("u_scale".to_owned(), 0.1),
            );
        }
        shader
            .insert_color_ref("u_house_color".to_owned(), self.color)
            .insert_color_ref("u_color".to_owned(), self.color)
            .insert_float_ref("u_card".to_owned(), 1.0)
            .insert_vec2_ref("u_box".to_owned(), vec2(1.0, 1.0))
            .insert_vec2_ref("u_align".to_owned(), vec2::ZERO);

        if let Some(description) = self
            .description
            .as_ref()
            .or(resources.definitions.get(name).map(|x| &x.description))
        {
            shader.insert_string_ref("u_description".to_owned(), description.clone(), 0);
        }
        let charges_str = match charges {
            Some(v) => format!("+{v}"),
            None => String::default(),
        };
        shader
            .after
            .push(resources.options.shaders.name.clone().insert_uniform(
                "u_text".to_owned(),
                ShaderUniform::String((1, format!("{name} {charges_str}"))),
            ));

        shader
    }
}
