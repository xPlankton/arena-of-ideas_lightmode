use super::*;

pub struct QueuedEffect {
    pub effect: Effect,
    pub caster: Option<Id>,
    pub target: Option<Id>,
}

impl Game {
    pub fn process_effects(&mut self) {
        while let Some(effect) = self.effects.pop() {
            match effect.effect {
                Effect::Damage {
                    hp,
                    lifesteal,
                    kill_effects,
                } => {
                    let target = effect
                        .target
                        .and_then(|id| self.units.get_mut(&id))
                        .expect("Target not found");
                    let mut damage = match hp {
                        DamageValue::Absolute(hp) => hp,
                        DamageValue::Relative(percent) => {
                            target.max_hp * percent / Health::new(100.0)
                        }
                    };
                    damage = min(damage, target.hp);
                    if damage > Health::new(0.0) {
                        if let Some((index, _)) = target
                            .statuses
                            .iter()
                            .enumerate()
                            .find(|(_, status)| matches!(status, Status::Shield))
                        {
                            damage = Health::new(0.0);
                            target.statuses.remove(index);
                        }
                    }
                    if damage > Health::new(0.0) {
                        target
                            .statuses
                            .retain(|status| !matches!(status, Status::Freeze));
                    }
                    let old_hp = target.hp;
                    target.hp -= damage;
                    self.render
                        .add_text(target.position, &format!("-{}", damage), Color::RED);
                    if old_hp > Health::new(0.0) && target.hp <= Health::new(0.0) {
                        // self.render.add_text(target.position, "KILL", Color::RED);
                        for kill_effect in kill_effects {
                            self.effects.push(QueuedEffect {
                                effect: kill_effect.clone(),
                                caster: effect.caster,
                                target: effect.target,
                            });
                        }
                    }

                    // Lifesteal
                    let lifesteal = match lifesteal {
                        DamageValue::Absolute(hp) => hp,
                        DamageValue::Relative(percent) => damage * percent / Health::new(100.0),
                    };
                    if let Some(caster) = effect.caster.and_then(|id| self.units.get_mut(&id)) {
                        caster.hp = (caster.hp + lifesteal).min(caster.max_hp);
                    }
                }
                Effect::AddStatus { status } => {
                    let target = effect
                        .target
                        .and_then(|id| self.units.get_mut(&id))
                        .expect("Target not found");
                    self.render
                        .add_text(target.position, status.name(), Color::BLUE);
                    target.statuses.push(status.clone());
                }
                Effect::Suicide => {
                    if let Some(caster) = effect.caster.and_then(|id| self.units.get_mut(&id)) {
                        caster.hp = Health::new(0.0);
                    }
                }
                Effect::Spawn { unit_type } => {
                    let caster = effect
                        .caster
                        .and_then(|id| self.units.get(&id).or(self.dead_units.get(&id)))
                        .expect("Caster not found");
                    let faction = caster.faction;
                    let target = effect
                        .target
                        .and_then(|id| self.units.get(&id).or(self.dead_units.get(&id)))
                        .expect("Target not found");
                    let position = target.position;
                    self.spawn_unit(&unit_type, faction, position);
                }
                Effect::TimeBomb { time, effects } => {
                    let target = effect
                        .target
                        .and_then(|id| self.units.get(&id).or(self.dead_units.get(&id)))
                        .expect("Target not found");
                    self.time_bombs.insert(TimeBomb {
                        id: self.next_id,
                        position: target.position,
                        caster: effect.caster,
                        time,
                        effects,
                    });
                    self.next_id += 1;
                }
                Effect::AOE {
                    radius,
                    filter,
                    effects,
                } => {
                    let caster = effect
                        .caster
                        .and_then(|id| self.units.get(&id).or(self.dead_units.get(&id)))
                        .expect("Caster not found");
                    let caster_faction = caster.faction;
                    let center = effect
                        .target
                        .and_then(|id| {
                            self.units
                                .get(&id)
                                .map(|unit| unit.position)
                                .or(self.dead_time_bombs.get(&id).map(|bomb| bomb.position))
                        })
                        .expect("Target not found");
                    self.render.add_text(center, "AOE", Color::RED);
                    for unit in &self.units {
                        if (unit.position - center).len() - unit.radius() > radius {
                            continue;
                        }
                        match filter {
                            TargetFilter::Allies => {
                                if unit.faction != caster_faction {
                                    continue;
                                }
                            }
                            TargetFilter::Enemies => {
                                if unit.faction == caster_faction {
                                    continue;
                                }
                            }
                            TargetFilter::All => {}
                        }
                        for new_effect in &effects {
                            self.effects.push(QueuedEffect {
                                effect: new_effect.clone(),
                                caster: effect.caster,
                                target: Some(unit.id),
                            });
                        }
                    }
                }
            }
        }
    }
}
