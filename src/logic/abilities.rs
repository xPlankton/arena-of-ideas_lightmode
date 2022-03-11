use super::*;

impl Logic<'_> {
    pub fn process_abilities(&mut self) {
        for unit in &mut self.model.units {
            if let Some(time) = &mut unit.ability_cooldown {
                *time -= self.delta_time;
                if *time < Time::new(0.0) {
                    unit.ability_cooldown = None;
                }
            }
        }
        for key in self.pressed_keys.drain(..) {
            for unit in &mut self.model.units {
                let template = &self.model.unit_templates[&unit.unit_type];
                if unit.ability_cooldown.is_some() {
                    continue;
                }
                if unit.faction != Faction::Player {
                    continue;
                }
                if unit
                    .all_statuses
                    .iter()
                    .any(|status| matches!(status, Status::Freeze | Status::Stun { .. }))
                {
                    continue;
                }
                let ability = match template.abilities.get(&key) {
                    Some(ability) => ability,
                    None => continue,
                };
                unit.ability_cooldown = Some(ability.cooldown);
                self.effects.push(QueuedEffect {
                    effect: ability.effect.clone(),
                    caster: Some(unit.id),
                    target: Some(unit.id),
                });
            }
        }
    }
}
