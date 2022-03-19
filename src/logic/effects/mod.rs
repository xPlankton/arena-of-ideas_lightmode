use super::*;

mod add_targets;
mod aoe;
mod attach_status;
mod chain;
mod change_context;
mod condition;
mod damage;
mod heal;
mod if_effect;
mod instant_action;
mod maybe_modify;
mod modifiers;
mod projectile;
mod spawn;
mod suicide;
mod time_bomb;

pub struct QueuedEffect<T> {
    pub effect: T,
    pub context: EffectContext,
}

#[derive(Debug, Clone, Copy)]
pub struct EffectContext {
    pub caster: Option<Id>,
    pub from: Option<Id>,
    pub target: Option<Id>,
}

impl EffectContext {
    pub fn get(&self, who: Who) -> Option<Id> {
        match who {
            Who::Caster => self.caster,
            Who::Target => self.target,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeighedEffect {
    pub weight: f32,
    #[serde(flatten)]
    pub effect: Effect,
}

impl Effect {
    pub fn walk_mut(&mut self, mut f: &mut impl FnMut(&mut Effect)) {
        match self {
            Self::Noop => {}
            Self::InstantAction => {}
            Self::Projectile(effect) => effect.walk_children_mut(f),
            Self::Damage(effect) => effect.walk_children_mut(f),
            Self::AttachStatus(effect) => effect.walk_children_mut(f),
            Self::Spawn(effect) => effect.walk_children_mut(f),
            Self::AOE(effect) => effect.walk_children_mut(f),
            Self::TimeBomb(effect) => effect.walk_children_mut(f),
            Self::Suicide(effect) => effect.walk_children_mut(f),
            Self::Chain(effect) => effect.walk_children_mut(f),
            Self::AddTargets(effect) => effect.walk_children_mut(f),
            Self::Repeat { effect, .. } => effect.walk_mut(f),
            Self::Random { choices } => {
                for choice in choices {
                    choice.effect.walk_mut(f);
                }
            }
            Self::List { effects } => {
                for effect in effects {
                    effect.walk_mut(f);
                }
            }
            Self::If(effect) => effect.walk_children_mut(f),
            Self::MaybeModify(effect) => effect.walk_children_mut(f),
            Self::ChangeContext(effect) => effect.walk_children_mut(f),
            Self::Heal(effect) => effect.walk_children_mut(f),
        }
        f(self);
    }
}

impl Default for Effect {
    fn default() -> Self {
        Self::Noop
    }
}

impl Logic<'_> {
    pub fn process_effects(&mut self) {
        while let Some(QueuedEffect { effect, context }) = self.effects.pop_front() {
            match effect {
                Effect::Noop => {}
                Effect::InstantAction => self.process_instant_action(context),
                Effect::Damage(effect) => self.process_damage_effect(QueuedEffect {
                    effect: *effect,
                    context,
                }),
                Effect::Projectile(effect) => self.process_projectile_effect(QueuedEffect {
                    effect: *effect,
                    context,
                }),
                Effect::AttachStatus(effect) => self.process_attach_status_effect(QueuedEffect {
                    effect: *effect,
                    context,
                }),
                Effect::Suicide(effect) => self.process_suicide_effect(QueuedEffect {
                    effect: *effect,
                    context,
                }),
                Effect::Spawn(effect) => self.process_spawn_effect(QueuedEffect {
                    effect: *effect,
                    context,
                }),
                Effect::TimeBomb(effect) => self.process_time_bomb_effect(QueuedEffect {
                    effect: *effect,
                    context,
                }),
                Effect::AOE(effect) => self.process_aoe_effect(QueuedEffect {
                    effect: *effect,
                    context,
                }),
                Effect::AddTargets(effect) => self.process_add_targets_effect(QueuedEffect {
                    effect: *effect,
                    context,
                }),
                Effect::Chain(effect) => self.process_chain_effect(QueuedEffect {
                    effect: *effect,
                    context,
                }),
                Effect::Repeat { times, effect } => {
                    for _ in 0..times {
                        self.effects.push_back(QueuedEffect {
                            effect: (*effect).clone(),
                            context,
                        });
                    }
                }
                Effect::List { effects } => {
                    for effect in effects {
                        self.effects.push_back(QueuedEffect { effect, context });
                    }
                }
                Effect::Random { choices } => {
                    let effect = choices
                        .choose_weighted(&mut global_rng(), |choice| choice.weight)
                        .unwrap()
                        .effect
                        .clone();
                    self.effects.push_back(QueuedEffect { effect, context });
                }
                Effect::If(effect) => self.process_if_effect(QueuedEffect {
                    effect: *effect,
                    context,
                }),
                Effect::MaybeModify(effect) => self.process_maybe_modify_effect(QueuedEffect {
                    effect: *effect,
                    context,
                }),
                Effect::ChangeContext(effect) => self.process_change_context_effect(QueuedEffect {
                    effect: *effect,
                    context,
                }),
                Effect::Heal(effect) => self.process_heal_effect(QueuedEffect {
                    effect: *effect,
                    context,
                }),
            }
        }
    }
}
