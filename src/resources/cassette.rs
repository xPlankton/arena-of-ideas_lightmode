use geng::prelude::itertools::Itertools;

use super::*;

pub struct Cassette {
    pub head: Time,
    queue: Vec<CassetteNode>,
    pub node_template: CassetteNode, // any new node will be cloned from this
    pub parallel_node: CassetteNode, // this node is always rendered
}

impl Default for Cassette {
    fn default() -> Self {
        Self {
            head: default(),
            queue: vec![default()],
            node_template: default(),
            parallel_node: default(),
        }
    }
}

const DEFAULT_EFFECT_KEY: &str = "default";

impl Cassette {
    pub fn close_node(&mut self) {
        let node = self.queue.last_mut().unwrap();
        let start = (node.start + node.duration).max(self.head);
        if node.duration == 0.0 {
            node.start = start;
            self.queue.pop();
        }
        let mut new_node = self.node_template.clone();
        new_node.start = start;
        self.queue.push(new_node);
    }

    pub fn merge_template_into_last(&mut self) {
        let node = self.queue.last_mut().unwrap();
        node.merge_mut(&self.node_template);
    }

    pub fn add_effect(&mut self, effect: VisualEffect) {
        self.add_effect_by_key(DEFAULT_EFFECT_KEY, effect);
    }

    pub fn add_effect_by_key(&mut self, key: &str, mut effect: VisualEffect) {
        let mut last = self.queue.last_mut().unwrap();
        if self.head > last.start + last.duration {
            self.close_node();
            last = self.queue.last_mut().unwrap();
        }
        if self.head > last.start && self.head < last.start + last.duration {
            effect.delay += self.head - last.start;
        }
        last.add_effect_by_key(key, effect);
    }

    pub fn get_key_count(&self, key: &str) -> usize {
        self.queue.last().unwrap().get_key_count(key)
    }

    pub fn add_entity_shader(&mut self, entity: legion::Entity, shader: Shader) {
        self.queue
            .last_mut()
            .unwrap()
            .add_entity_shader(entity, shader);
    }

    pub fn get_shaders(
        resources: &mut Resources,
        mut world_shaders: HashMap<legion::Entity, Shader>,
    ) -> Vec<Shader> {
        let cassette = &resources.cassette;
        let mut node = cassette
            .get_node_at_ts(cassette.head)
            .merge(&cassette.parallel_node);
        let time = cassette.head - node.start;
        world_shaders.extend(node.entity_shaders.clone().into_iter());
        let mut entity_shaders = world_shaders;

        // 1st phase: apply any changes to entity shaders uniforms
        for effect in node.effects.values().flatten().sorted_by_key(|x| x.order) {
            let time = time - effect.delay;
            if effect.duration > 0.0 && (time > effect.duration || time < 0.0) {
                continue;
            }
            let effect_type = &effect.r#type;
            match effect_type {
                VisualEffectType::EntityShaderAnimation { .. }
                | VisualEffectType::EntityShaderConst { .. } => {
                    effect_type.process(time / effect.duration, &mut entity_shaders);
                }
                _ => {}
            };
        }
        UnitSystem::inject_entity_shaders_uniforms(&mut entity_shaders, resources);
        StatusSystem::add_active_statuses_panel_to_node(&mut node, resources);

        // 2nd phase: apply any other shaders that might need updated entity shaders uniforms
        let mut extra_shaders: Vec<Shader> = default();
        for effect in node.effects.values().flatten().sorted_by_key(|x| x.order) {
            let time = time - effect.delay;
            if effect.duration > 0.0 && (time > effect.duration || time < 0.0) {
                continue;
            }
            let effect_type = &effect.r#type;
            match effect_type {
                VisualEffectType::EntityShaderAnimation { .. }
                | VisualEffectType::EntityShaderConst { .. } => {}
                _ => {
                    extra_shaders
                        .extend(effect_type.process(time / effect.duration, &mut entity_shaders));
                }
            };
        }

        let mut entity_shaders_vec = entity_shaders
            .into_iter()
            .sorted_by_key(|(entity, shader)| {
                (shader.layer.index(), shader.order, format!("{:?}", entity))
            })
            .collect_vec();

        let mut hovered_entity = None;
        for (entity, shader) in entity_shaders_vec.iter().rev() {
            if let Some(area) = AreaComponent::from_shader(shader) {
                if area.contains(resources.input.mouse_pos) {
                    hovered_entity = Some(*entity);
                    break;
                }
            }
        }
        if let Some(hovered) = InputSystem::set_hovered_entity(hovered_entity, resources) {
            let last_ind = entity_shaders_vec.len() - 1;
            if let Some(hovered_ind) = entity_shaders_vec.iter().position(|x| x.0 == hovered) {
                entity_shaders_vec.swap(hovered_ind, last_ind);
            }
        }
        let entity_shaders_vec = entity_shaders_vec
            .into_iter()
            .map(|(_, shader)| shader)
            .collect_vec();

        [entity_shaders_vec, extra_shaders].concat()
    }

    pub fn length(&self) -> Time {
        let last = self.queue.last().unwrap();
        last.start + last.duration
    }

    pub fn last_start(&self) -> Time {
        self.queue.last().unwrap().start
    }

    pub fn clear(&mut self) {
        self.queue = vec![default()];
        self.head = 0.0;
        self.node_template.clear();
        self.parallel_node.clear();
    }

    fn get_node_at_ts(&self, ts: Time) -> &CassetteNode {
        if ts > self.length() {
            return &self.node_template;
        }
        let index = match self
            .queue
            .binary_search_by_key(&r32(ts), |node| r32(node.start))
        {
            Ok(index) => index,
            Err(index) => index - 1,
        };
        if let Some(node) = self.queue.get(index) {
            node
        } else {
            &self.node_template
        }
    }
}

#[derive(Default, Clone, Debug)]

pub struct CassetteNode {
    start: Time,
    duration: Time,
    pub entity_shaders: HashMap<legion::Entity, Shader>,
    active_statuses: HashMap<legion::Entity, HashMap<String, Context>>,
    effects: HashMap<String, Vec<VisualEffect>>,
}

impl CassetteNode {
    pub fn add_entity_shader(&mut self, entity: legion::Entity, shader: Shader) {
        self.entity_shaders.insert(entity, shader);
    }
    pub fn add_effect_by_key(&mut self, key: &str, effect: VisualEffect) {
        self.duration = self.duration.max(effect.duration + effect.delay);
        let mut vec = self.effects.remove(key).unwrap_or_default();
        vec.push(effect);
        self.effects.insert(key.to_string(), vec);
    }
    pub fn add_effects_by_key(&mut self, key: &str, effects: Vec<VisualEffect>) {
        effects
            .into_iter()
            .for_each(|effect| self.add_effect_by_key(key, effect))
    }
    pub fn get_key_count(&self, key: &str) -> usize {
        match self.effects.get(key).and_then(|v| Some(v.len())) {
            Some(value) => value,
            None => 0,
        }
    }
    pub fn clear_key(&mut self, key: &str) {
        self.effects.remove(key);
    }
    pub fn clear(&mut self) {
        self.start = default();
        self.duration = default();
        self.entity_shaders.clear();
        self.effects.clear();
    }
    pub fn clear_entities(&mut self) {
        self.entity_shaders.clear();
    }
    pub fn merge(&self, other: &CassetteNode) -> CassetteNode {
        let mut node = self.clone();
        node.merge_mut(other);
        node
    }
    pub fn merge_mut(&mut self, other: &CassetteNode) {
        let mut node = self;
        node.duration = node.duration.max(node.duration);
        for (key, other_effects) in other.effects.iter() {
            if key == DEFAULT_EFFECT_KEY {
                let mut effects = node.effects.remove(key).unwrap_or_default();
                effects.extend(other_effects.iter().cloned());
                node.effects.insert(key.clone(), effects);
            } else {
                node.effects.insert(key.clone(), other_effects.clone());
            }
        }
        other.entity_shaders.iter().for_each(|(entity, shader)| {
            node.entity_shaders.insert(*entity, shader.clone());
        });
        other
            .active_statuses
            .iter()
            .for_each(|(entity, other_statuses)| {
                let mut statuses = node.active_statuses.remove(entity).unwrap_or_default();
                other_statuses.iter().for_each(|(name, context)| {
                    statuses.insert(name.clone(), context.clone());
                });
                node.active_statuses.insert(*entity, statuses);
            })
    }
    pub fn start(&self) -> Time {
        self.start
    }
    pub fn save_active_statuses(&mut self, pool: &StatusPool) {
        self.active_statuses = pool.active_statuses.clone();
    }
    pub fn get_entity_statuses_names(&self, entity: &legion::Entity) -> Vec<String> {
        self.active_statuses
            .get(entity)
            .and_then(|statuses| Some(statuses.keys().cloned().collect_vec()))
            .unwrap_or_else(|| vec![])
    }
}
