use super::*;

impl Logic<'_> {
    /// Spawns the unit and returns its id. If there is a unit in that position and there is an
    /// empty slot to the left, it and all units to the left are shifted to the left.
    /// Otherwise, if all slots are occupied, the unit is placed on top the unit in that position.
    pub fn spawn_unit(&mut self, unit_type: &UnitType, faction: Faction, position: Position) -> Id {
        let mut template = &self.model.unit_templates[unit_type];
        let id = self.model.next_id;

        let mut unit = Unit::new(
            &template,
            &mut self.model.next_id,
            unit_type.clone(),
            faction,
            position,
            &self.model.statuses,
        );
        for (clan, &clan_members) in &self.model.config.clans {
            clan.apply_effects(
                &mut unit,
                &self.model.clan_effects,
                clan_members,
                &mut self.model.next_id,
                &self.model.statuses,
            );
        }

        self.model.next_id += 1;
        self.model.spawning_units.insert(unit);
        id
    }
    pub fn process_spawns(&mut self) {
        let mut new_units = Vec::new();
        for unit in &mut self.model.spawning_units {
            if let Some(time) = &mut unit.spawn_animation_time_left {
                *time -= self.delta_time;
                if *time <= Time::new(0.0) {
                    unit.spawn_animation_time_left = None;
                    new_units.push(unit.clone());
                }
            }
        }
        for mut unit in new_units {
            for (effect, vars, status_id) in unit.all_statuses.iter().flat_map(|status| {
                status.trigger(|trigger| matches!(trigger, StatusTrigger::Spawn))
            }) {
                self.effects.push_front(QueuedEffect {
                    effect,
                    context: EffectContext {
                        caster: Some(unit.id),
                        from: Some(unit.id),
                        target: Some(unit.id),
                        vars,
                        status_id: Some(status_id),
                    },
                })
            }
            self.model.units.insert(unit);
        }
        self.model
            .spawning_units
            .retain(|unit| unit.spawn_animation_time_left.is_some());
    }
}
