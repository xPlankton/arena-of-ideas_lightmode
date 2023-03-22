use super::*;
use geng::ui::*;

pub struct BattleSystem {}

impl BattleSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run_battle(world: &mut legion::World, resources: &mut Resources) -> Vec<CassetteNode> {
        let mut ticks = 0;
        let nodes = &mut Some(default());
        while Self::tick(world, resources, nodes) && ticks < 1000 {
            ticks += 1;
        }
        nodes.to_owned().unwrap_or_default()
    }

    pub fn init_battle(world: &mut legion::World, resources: &mut Resources) {
        Self::clear_world(world, resources);
        Self::load_floor(resources, world);
        Self::load_player_team(resources, world);
        SlotSystem::fill_gaps(world, resources, &hashset! {Faction::Dark, Faction::Light});
    }

    pub fn battle_won(world: &legion::World) -> bool {
        <&UnitComponent>::query()
            .iter(world)
            .filter(|unit| unit.faction == Faction::Dark)
            .count()
            == 0
    }

    pub fn finish_battle(world: &mut legion::World, resources: &mut Resources) {
        resources.game_won = Self::battle_won(world);
        resources.last_round = resources.floors.current_ind();
        if !resources.game_won {
            resources.transition_state = GameState::GameOver;
        } else {
            if resources.floors.next() {
                resources.transition_state = GameState::Shop;
            } else {
                resources.transition_state = GameState::GameOver;
            }
        }
        Self::clear_world(world, resources);
    }

    pub fn clear_world(world: &mut legion::World, resources: &mut Resources) {
        let factions = &hashset! {Faction::Dark, Faction::Light};
        UnitSystem::clear_factions(world, resources, factions);
    }

    pub fn load_floor(resources: &mut Resources, world: &mut legion::World) {
        let team = resources.floors.current().clone();
        let faction = Faction::Dark;
        TeamPool::save_team(faction, team, resources);
        TeamPool::load_team(&faction, world, resources);
    }

    pub fn load_player_team(resources: &mut Resources, world: &mut legion::World) {
        let team = TeamPool::get_team(Faction::Team, resources).clone();
        let faction = Faction::Light;
        TeamPool::save_team(faction, team, resources);
        TeamPool::load_team(&faction, world, resources);
    }

    /// Refresh all units, add them as node entities, push node to nodes, clear node
    fn push_node(
        node: &mut CassetteNode,
        nodes: &mut Option<Vec<CassetteNode>>,
        world: &mut legion::World,
        resources: &Resources,
    ) {
        if node.duration == 0.0 {
            return;
        }
        if let Some(nodes) = nodes {
            let factions = &hashset! {Faction::Light, Faction::Dark};
            ContextSystem::refresh_factions(factions, world, resources);
            UnitSystem::draw_all_units_to_cassette_node(factions, node, world, resources);
            let mut push_node = CassetteNode::default();
            if let Some(last) = nodes.last() {
                node.start = last.duration + last.start;
            }
            mem::swap(node, &mut push_node);
            nodes.push(push_node);
        }
    }

    pub fn tick(
        world: &mut legion::World,
        resources: &mut Resources,
        nodes: &mut Option<Vec<CassetteNode>>,
    ) -> bool {
        let factions = &hashset! {Faction::Light, Faction::Dark};
        SlotSystem::fill_gaps(world, resources, factions);
        ContextSystem::refresh_factions(factions, world, resources);
        let node = &mut CassetteNode::default();
        Self::move_to_slots_animated(world, node);
        Self::push_node(node, nodes, world, resources);
        ActionSystem::run_ticks(world, resources)
            .into_iter()
            .for_each(|mut node| Self::push_node(&mut node, nodes, world, resources));
        if let Some((left, right)) = Self::find_hitters(world) {
            Self::move_strikers(&StrikePhase::Charge, left, right, world, node);
            Self::push_node(node, nodes, world, resources);
            Self::move_strikers(&StrikePhase::Release, left, right, world, node);
            Self::push_node(node, nodes, world, resources);
            Self::add_strike_vfx(world, resources, node);
            Self::push_node(node, nodes, world, resources);
            Self::hit(left, right, nodes, world, resources);
            Self::death_check(world, resources);
            Self::move_strikers(&StrikePhase::Retract, left, right, world, node);
            Self::push_node(node, nodes, world, resources);
            return true;
        }
        false
    }

    fn move_strikers(
        phase: &StrikePhase,
        left: legion::Entity,
        right: legion::Entity,
        world: &mut legion::World,
        node: &mut CassetteNode,
    ) {
        let (left_pos, right_pos) = Self::get_strikers_positions(phase);
        let (easing, duration) = match phase {
            StrikePhase::Charge => (EasingType::QuartInOut, 1.5),
            StrikePhase::Release => (EasingType::Linear, 0.1),
            StrikePhase::Retract => (EasingType::QuartOut, 0.25),
        };
        VfxSystem::translate_animated(left, left_pos, node, world, easing, duration);
        VfxSystem::translate_animated(right, right_pos, node, world, easing, duration);
    }

    fn get_strikers_positions(phase: &StrikePhase) -> (vec2<f32>, vec2<f32>) {
        let left = vec2(-1.0, 1.0);
        let right = vec2(1.0, 1.0);
        let left_slot = SlotSystem::get_position(1, &Faction::Light);
        let right_slot = SlotSystem::get_position(1, &Faction::Dark);

        let delta = match phase {
            StrikePhase::Charge => vec2(4.5, 0.0),
            StrikePhase::Release => vec2(-right_slot.x + 1.0, 0.0),
            StrikePhase::Retract => vec2::ZERO,
        };
        (delta * left + left_slot, delta * right + right_slot)
    }

    pub fn find_hitters(world: &legion::World) -> Option<(legion::Entity, legion::Entity)> {
        let units = <(&UnitComponent, &EntityComponent)>::query()
            .iter(world)
            .collect_vec();

        units
            .iter()
            .find(|(unit, _)| unit.slot == 1 && unit.faction == Faction::Light)
            .and_then(|(_, left)| {
                match units
                    .iter()
                    .find(|(unit, _)| unit.slot == 1 && unit.faction == Faction::Dark)
                {
                    Some((_, right)) => Some((left.entity, right.entity)),
                    None => None,
                }
            })
    }

    pub fn hit(
        left: legion::Entity,
        right: legion::Entity,
        nodes: &mut Option<Vec<CassetteNode>>,
        world: &mut legion::World,
        resources: &mut Resources,
    ) {
        let context_left = Context {
            owner: left,
            target: right,
            ..ContextSystem::get_context(left, world)
        };
        resources.action_queue.push_back(Action::new(
            context_left.clone(),
            Effect::Damage {
                value: None,
                then: None,
            }
            .wrap(),
        ));
        ActionSystem::run_ticks(world, resources)
            .into_iter()
            .for_each(|mut node| Self::push_node(&mut node, nodes, world, resources));
        Event::AfterStrike {
            owner: left,
            target: right,
        }
        .send(world, resources);
        ActionSystem::run_ticks(world, resources)
            .into_iter()
            .for_each(|mut node| Self::push_node(&mut node, nodes, world, resources));
        let context_right = Context {
            owner: right,
            target: left,
            ..ContextSystem::get_context(right, world)
        };
        resources.action_queue.push_back(Action::new(
            context_right.clone(),
            Effect::Damage {
                value: None,
                then: None,
            }
            .wrap(),
        ));
        ActionSystem::run_ticks(world, resources)
            .into_iter()
            .for_each(|mut node| Self::push_node(&mut node, nodes, world, resources));
        Event::AfterStrike {
            owner: right,
            target: left,
        }
        .send(world, resources);
        ActionSystem::run_ticks(world, resources)
            .into_iter()
            .for_each(|mut node| Self::push_node(&mut node, nodes, world, resources));
    }

    pub fn death_check(world: &mut legion::World, resources: &mut Resources) {
        ContextSystem::refresh_all(world, resources);
        while let Some(dead_unit) = <(&EntityComponent, &Context, &HealthComponent)>::query()
            .iter(world)
            .filter_map(|(unit, context, _)| {
                match context.vars.get_int(&VarName::HpValue)
                    <= context.vars.get_int(&VarName::HpDamage)
                {
                    true => Some(unit.entity),
                    false => None,
                }
            })
            .choose(&mut thread_rng())
        {
            resources.logger.log(
                &format!("Entity#{:?} dead", dead_unit),
                &LogContext::UnitCreation,
            );
            if UnitSystem::process_death(dead_unit, world, resources) {
                resources.logger.log(
                    &format!("Entity#{:?} removed", dead_unit),
                    &LogContext::UnitCreation,
                );
            }
        }
    }

    fn move_to_slots_animated(world: &mut legion::World, node: &mut CassetteNode) {
        UnitSystem::collect_factions(world, &hashset! { Faction::Light, Faction::Dark })
            .into_iter()
            .for_each(|(entity, unit)| {
                VfxSystem::translate_animated(
                    entity,
                    SlotSystem::get_unit_position(&unit),
                    node,
                    world,
                    EasingType::CubicIn,
                    0.2,
                )
            });
    }

    fn add_strike_vfx(world: &legion::World, resources: &mut Resources, node: &mut CassetteNode) {
        let position = BATTLEFIELD_POSITION;
        node.add_effect(VfxSystem::vfx_strike(resources, position));
    }
}

impl System for BattleSystem {
    fn ui<'a>(
        &'a mut self,
        cx: &'a ui::Controller,
        world: &'a legion::World,
        resources: &'a Resources,
    ) -> Box<dyn ui::Widget + 'a> {
        Box::new(
            (Text::new(
                format!("Round #{}", resources.floors.current_ind()),
                resources.fonts.get_font(0),
                70.0,
                Rgba::WHITE,
            ),)
                .column()
                .flex_align(vec2(Some(1.0), None), vec2(1.0, 1.0))
                .uniform_padding(32.0),
        )
    }

    fn update(&mut self, world: &mut legion::World, resources: &mut Resources) {}
}

enum StrikePhase {
    Charge,
    Release,
    Retract,
}
