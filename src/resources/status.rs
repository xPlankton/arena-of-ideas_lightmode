use geng::prelude::itertools::Itertools;

use super::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Status {
    pub trigger: Trigger,
    pub description: Option<String>,
    #[serde(default = "default_color")]
    pub color: Rgba<f32>,
    pub shader: Option<Shader>,
}

fn default_color() -> Rgba<f32> {
    Rgba::WHITE
}

#[derive(Default, Debug)]
pub struct StatusPool {
    pub defined_statuses: HashMap<String, Status>, // status name -> status
    pub active_statuses: HashMap<legion::Entity, HashMap<String, i32>>, // entity -> status name -> charges
    pub status_changes: VecDeque<(legion::Entity, String, i32)>, // added or removed status charges
}

impl StatusPool {
    /// Send event to all active statuses
    pub fn notify_all(
        event: &Event,
        factions: &HashSet<Faction>,
        resources: &mut Resources,
        world: &legion::World,
    ) {
        <(&EntityComponent, &Trigger, &UnitComponent)>::query()
            .iter(world)
            .filter(|(_, _, unit)| factions.contains(&unit.faction))
            .for_each(|(entity, _, _)| {
                Self::notify_entity(event, entity.entity, resources, world, None)
            });
    }

    /// Trigger all active statuses on entity
    pub fn notify_entity(
        event: &Event,
        entity: legion::Entity,
        resources: &mut Resources,
        world: &legion::World,
        context: Option<Context>,
    ) {
        let context = context.unwrap_or_else(|| ContextSystem::get_context(entity, world));
        for (name, trigger, charges) in resources.status_pool.collect_triggers(&entity, world) {
            trigger.catch_event(
                event,
                &mut resources.action_queue,
                context
                    .clone()
                    .add_var(VarName::StatusName, Var::String((0, name.clone())))
                    .add_var(VarName::Charges, Var::Int(charges))
                    .to_owned(),
                &resources.logger,
            );
        }
    }

    pub fn calculate_entity(
        event: &Event,
        entity: legion::Entity,
        context: Context,
        world: &legion::World,
        resources: &Resources,
    ) -> Context {
        let mut context = context.to_owned();
        for (_, trigger, charges) in resources.status_pool.collect_triggers(&entity, world) {
            context.add_var(VarName::Charges, Var::Int(charges));
            context = trigger.calculate_event(event, context, world, resources);
        }
        context.vars.remove(&VarName::Charges);
        context
    }

    pub fn collect_triggers(
        &self,
        entity: &legion::Entity,
        world: &legion::World,
    ) -> Vec<(String, Trigger, i32)> {
        self.collect_statuses(&entity)
            .into_iter()
            .map(|(name, status, charges)| (name, status.trigger, charges))
            .chain(
                world
                    .entry_ref(*entity)
                    .iter()
                    .filter_map(|x| x.get_component::<Trigger>().ok())
                    .map(|trigger| ("_local".to_string(), (*trigger).clone(), 1)),
            )
            .collect_vec()
    }

    pub fn collect_statuses(&self, entity: &legion::Entity) -> Vec<(String, Status, i32)> {
        self.active_statuses
            .get(entity)
            .unwrap_or(&default())
            .iter()
            .map(|(name, charges)| {
                (
                    name.clone(),
                    self.defined_statuses
                        .get(name)
                        .expect(&format!("Status undefined: {}", name))
                        .clone(),
                    *charges,
                )
            })
            .collect_vec()
    }

    pub fn get_entity_shaders(&self, entity: &legion::Entity) -> Vec<Shader> {
        self.collect_statuses(entity)
            .into_iter()
            .filter_map(|(_, status, _)| match status.shader {
                Some(mut shader) => {
                    shader.parameters.uniforms.insert(
                        VarName::Color.convert_to_uniform(),
                        ShaderUniform::Color(status.color),
                    );
                    Some(shader)
                }
                None => None,
            })
            .collect_vec()
    }

    pub fn change_entity_status(
        entity: legion::Entity,
        status_name: &String,
        resources: &mut Resources,
        charges_change: i32,
    ) {
        if charges_change != 0 {
            resources.logger.log(
                &format!(
                    "Change status {:?} {} {}",
                    entity, status_name, charges_change
                ),
                &LogContext::Effect,
            );
            resources.status_pool.status_changes.push_back((
                entity,
                status_name.to_string(),
                charges_change,
            ))
        }
    }

    fn add_status_charge(
        entity: legion::Entity,
        status: &String,
        resources: &mut Resources,
        world: &legion::World,
    ) {
        let mut entity_statuses = resources
            .status_pool
            .active_statuses
            .remove(&entity)
            .unwrap_or_default();
        let charges = *entity_statuses.get(status).unwrap_or(&0);
        if charges == 0 {
            Event::StatusAdd {
                status: status.to_string(),
                owner: entity,
            }
            .send(world, resources);
        }
        Event::StatusChargeAdd {
            status: status.to_string(),
            owner: entity,
        }
        .send(world, resources);
        entity_statuses.insert(status.to_string(), charges + 1);
        resources
            .status_pool
            .active_statuses
            .insert(entity, entity_statuses);
    }

    fn remove_status_charge(
        entity: legion::Entity,
        status: &String,
        resources: &mut Resources,
        world: &legion::World,
    ) {
        let mut entity_statuses = resources
            .status_pool
            .active_statuses
            .remove(&entity)
            .unwrap_or_default();
        let charges = *entity_statuses.get(status).unwrap_or(&0);
        if charges == 1 {
            Event::StatusRemove {
                status: status.to_string(),
                owner: entity,
            }
            .send(world, resources);
        }
        Event::StatusChargeRemove {
            status: status.to_string(),
            owner: entity,
        }
        .send(world, resources);
        if charges > 1 {
            entity_statuses.insert(status.to_string(), charges - 1);
        } else {
            entity_statuses.remove(status);
        }
        resources
            .status_pool
            .active_statuses
            .insert(entity, entity_statuses);
    }

    pub fn process_status_changes(
        world: &legion::World,
        resources: &mut Resources,
        node: &mut Option<CassetteNode>,
    ) {
        let max_delay = 0.5;
        let delay_per_charge = max_delay / resources.status_pool.status_changes.len() as f32;
        let key = "status_changes";
        let mut cnt = 0;
        while let Some((entity, status_name, charges_delta)) =
            resources.status_pool.status_changes.pop_front()
        {
            for _ in 0..charges_delta.abs() {
                let (text, color) = if charges_delta > 0 {
                    Self::add_status_charge(entity, &status_name, resources, world);
                    ("+", resources.options.colors.text_add_color)
                } else {
                    Self::remove_status_charge(entity, &status_name, resources, world);
                    ("-", resources.options.colors.text_remove_color)
                };
                if let Some(node) = node.as_mut() {
                    let text = format!("{}{}", text, &status_name);
                    let outline_color = resources
                        .status_pool
                        .defined_statuses
                        .get(&status_name)
                        .unwrap()
                        .color;
                    node.add_effect_by_key(
                        key,
                        VfxSystem::vfx_show_text(
                            resources,
                            &text,
                            color,
                            outline_color,
                            ContextSystem::try_get_position(entity, world).unwrap(),
                            1,
                            delay_per_charge * cnt as f32,
                        ),
                    );
                }
            }
            cnt += 1;
        }
    }

    pub fn define_status(name: String, status: Status, resources: &mut Resources) {
        if let Some(description) = status.description.as_ref() {
            resources
                .definitions
                .insert(name.clone(), status.color, description.clone());
        }
        resources.status_pool.defined_statuses.insert(name, status);
    }

    pub fn clear_all_active(resources: &mut Resources) {
        resources.status_pool.active_statuses.clear();
    }

    pub fn clear_entity(entity: &legion::Entity, resources: &mut Resources) {
        resources.status_pool.active_statuses.remove(entity);
    }

    pub fn clear_entity_by_changes(entity: &legion::Entity, resources: &mut Resources) {
        let pool = &mut resources.status_pool;
        if let Some(statuses) = pool.active_statuses.get(entity) {
            for (name, charges) in statuses.into_iter() {
                pool.status_changes
                    .push_back((*entity, name.clone(), -charges));
            }
        }
    }
}
