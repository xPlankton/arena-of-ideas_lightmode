use legion::EntityStore;

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Effect {
    Damage {
        value: Option<ExpressionInt>,
        on_hit: Option<Box<EffectWrapped>>,
    },
    Heal {
        value: Box<ExpressionInt>,
    },
    Kill,
    Repeat {
        count: ExpressionInt,
        effect: Box<EffectWrapped>,
    },
    List {
        effects: Vec<Box<EffectWrapped>>,
    },
    Debug {
        message: String,
    },
    Noop,
    SetVarInt {
        var: VarName,
        value: ExpressionInt,
    },
    SetVarFaction {
        var: VarName,
        value: ExpressionFaction,
    },
    ChangeAbilityVarInt {
        house: HouseName,
        ability: String,
        var: VarName,
        delta: ExpressionInt,
    },
    AddStatus {
        name: String,
    },
    RemoveStatus {
        name: String,
    },
    RemoveThisStatus,
    ClearStatuses,
    ChangeStatus {
        name: String,
        charges: ExpressionInt,
    },
    UseAbility {
        house: HouseName,
        name: String,
    },
    SetHealth {
        value: ExpressionInt,
    },
    SetAttack {
        value: ExpressionInt,
    },
    SetFaction {
        faction: ExpressionFaction,
    },
    SetSlot {
        slot: ExpressionInt,
    },
    TakeVar {
        var: VarName,
        new_name: Option<VarName>,
        entity: ExpressionEntity,
        effect: Box<EffectWrapped>,
    },
    If {
        condition: Condition,
        then: Box<EffectWrapped>,
        r#else: Option<Box<EffectWrapped>>,
    },
    ShowText {
        text: String,
        color: Option<Rgba<f32>>,
    },
    ShowCurve {
        color: Option<Rgba<f32>>,
    },
    Aoe {
        factions: Vec<Faction>,
        effect: Box<EffectWrapped>,
    },
    Revive {
        slot: Option<ExpressionInt>,
    },
    RemoveTrigger,
    /// Do effect if a unit matches condition
    FindTarget {
        faction: ExpressionFaction,
        condition: Condition,
        effect: Box<EffectWrapped>,
    },
}

impl Effect {
    pub fn wrap(self) -> EffectWrapped {
        EffectWrapped {
            effect: self,
            target: default(),
            owner: default(),
            after: default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EffectWrapped {
    #[serde(flatten)]
    pub effect: Effect,
    pub target: Option<ExpressionEntity>,
    pub owner: Option<ExpressionEntity>,
    pub after: Option<Box<EffectWrapped>>,
}

impl EffectWrapped {
    pub fn process(
        &self,
        context: Context,
        world: &mut legion::World,
        resources: &mut Resources,
    ) -> Result<CassetteNode, Error> {
        let mut node = CassetteNode::default();
        let mut context = Context {
            target: self
                .target
                .as_ref()
                .and_then(|t| t.calculate(&context, world, resources).ok())
                .unwrap_or(context.target),
            owner: self
                .owner
                .as_ref()
                .and_then(|t| t.calculate(&context, world, resources).ok())
                .unwrap_or(context.owner),
            ..context
        };
        match &self.effect {
            Effect::Damage {
                value,
                on_hit: then,
            } => {
                let mut value = match value {
                    Some(v) => v.calculate(&context, world, resources)?,
                    None => context.vars.get_int(&VarName::AttackValue),
                } as usize;
                context.vars.insert(VarName::Damage, Var::Int(value as i32));
                context = Event::ModifyOutgoingDamage { context }.calculate(world, resources);
                let initial_damage = context.vars.get_int(&VarName::Damage).max(0) as usize;
                Event::BeforeOutgoingDamage {
                    context: context.clone(),
                }
                .send(world, resources);
                Event::BeforeIncomingDamage {
                    context: context.clone(),
                }
                .send(world, resources);
                context = Event::ModifyIncomingDamage { context }.calculate(world, resources);
                value = context.vars.get_int(&VarName::Damage).max(0) as usize;
                let text = format!("-{}", value);
                let mut target = world
                    .entry(context.target)
                    .context("Failed to get Target")?;
                node.add_effect(VfxSystem::vfx_show_text(
                    resources,
                    &text,
                    resources.options.colors.text_remove_color,
                    resources.options.colors.damage_text,
                    target.get_component::<AreaComponent>().unwrap().position,
                    0,
                    0.0,
                ));
                if value > 0 {
                    let hp = target.get_component_mut::<HealthComponent>()?;
                    hp.damage += value as usize;
                    node.add_effect(VisualEffect::new(
                        1.0,
                        VisualEffectType::EntityShaderAnimation {
                            entity: context.target,
                            from: hashmap! {
                                "u_damage_taken" => ShaderUniform::Float(1.0),
                            }
                            .into(),
                            to: hashmap! {
                                "u_damage_taken" => ShaderUniform::Float(0.0),
                            }
                            .into(),
                            easing: EasingType::Linear,
                        },
                        0,
                    ));
                    resources.logger.log(
                        &format!("Entity#{:?} {} damage taken", context.target, value),
                        &LogContext::Effect,
                    );
                    resources
                        .unit_offenders
                        .insert(context.target, context.owner);
                    if let Some(effect) = then {
                        resources
                            .action_queue
                            .push_front(Action::new(context.clone(), effect.deref().clone()));
                    }
                    Event::AfterDamageDealt {
                        context: context.clone(),
                    }
                    .send(world, resources);
                }
                context.add_var(VarName::Damage, Var::Int(initial_damage as i32));
                Event::AfterOutgoingDamage {
                    context: context.clone(),
                }
                .send(world, resources);
                Event::AfterIncomingDamage {
                    context: context.clone(),
                }
                .send(world, resources);
            }
            Effect::Heal { value } => {
                let value = value.calculate(&context, world, resources)? as usize;
                let text = format!("+{}", value);
                let mut target = world
                    .entry(context.target)
                    .context("Failed to get Target")?;
                if let Some(hp) = target.get_component_mut::<HealthComponent>().ok() {
                    let value = value.min(hp.damage);
                    hp.damage -= value;
                    node.add_effect(VfxSystem::vfx_show_text(
                        resources,
                        &text,
                        resources.options.colors.text_add_color,
                        resources.options.colors.damage_text,
                        target.get_component::<AreaComponent>().unwrap().position,
                        0,
                        0.0,
                    ));
                }
            }
            Effect::Repeat { count, effect } => {
                for _ in 0..count.calculate(&context, world, resources)? {
                    resources
                        .action_queue
                        .push_front(Action::new(context.clone(), effect.deref().clone()));
                }
            }
            Effect::Debug { message } => debug!("Debug effect: {}", message),
            Effect::Noop => {}
            Effect::List { effects } => effects.iter().rev().for_each(|effect| {
                resources
                    .action_queue
                    .push_front(Action::new(context.clone(), effect.deref().clone()))
            }),
            Effect::SetVarInt { var, value } => {
                let value = value.calculate(&context, world, resources)?;
                context.add_var(*var, Var::Int(value));
            }
            Effect::SetVarFaction { var, value } => {
                let value = value.calculate(&context, world, resources)?;
                context.add_var(*var, Var::Faction(value));
            }
            Effect::ChangeStatus { name, .. }
            | Effect::AddStatus { name, .. }
            | Effect::RemoveStatus { name, .. } => {
                let charges = match &self.effect {
                    Effect::AddStatus { name: _ } => 1,
                    Effect::RemoveStatus { name: _ } => -1,
                    Effect::ChangeStatus { name: _, charges } => {
                        charges.calculate(&context, world, resources)?
                    }
                    _ => 0,
                };
                StatusPool::change_entity_status(context.target, &name, resources, charges);
            }
            Effect::RemoveThisStatus => {
                StatusPool::change_entity_status(
                    context.target,
                    &context.vars.get_string(&VarName::StatusName),
                    resources,
                    -1,
                );
            }
            Effect::ClearStatuses => {
                resources.status_pool.clear_entity(&context.target);
            }
            Effect::UseAbility { name, house } => {
                let owner_entry = world
                    .entry_ref(context.owner)
                    .context("Failed to get Owner")?;
                if owner_entry
                    .get_component::<HouseComponent>()?
                    .houses
                    .get(&house)
                    .is_none()
                {
                    panic!(
                        "Tried to use {} while not being a member of the {:?}",
                        name, house
                    );
                }
                let defaults = resources
                    .house_pool
                    .try_get_ability_vars(house, name)
                    .context("Failed to find ability")?;
                let faction = Faction::from_entity(context.owner, world, &resources);
                context.vars.merge_mut(defaults, true);
                if let Some(overrides) = TeamPool::try_get_team(faction, resources)
                    .and_then(|x| x.ability_state.get_vars(house, name))
                {
                    context.vars.merge_mut(overrides, true);
                }
                context.vars.insert(
                    VarName::Color,
                    Var::Color(resources.house_pool.get_color(house)),
                );
                let effect = EffectWrapped {
                    effect: Effect::ShowText {
                        text: format!("Use {}", name),
                        color: None,
                    },
                    target: None,
                    owner: None,
                    after: Some(Box::new(
                        resources.house_pool.get_ability(house, name).effect.clone(),
                    )),
                };
                resources
                    .action_queue
                    .push_front(Action::new(context.clone(), effect));
            }
            Effect::SetHealth { value } => {
                let value = value.calculate(&context, world, resources)?;
                let mut target = world.entry(context.target).unwrap();
                target.get_component_mut::<HealthComponent>().unwrap().value = value;
            }
            Effect::SetAttack { value } => {
                let value = value.calculate(&context, world, resources)? as usize;
                let mut target = world.entry(context.target).unwrap();
                target.get_component_mut::<AttackComponent>().unwrap().value = value;
            }
            Effect::ChangeAbilityVarInt {
                house,
                ability,
                var,
                delta,
            } => {
                let delta = delta.calculate(&context, world, resources)?;
                resources.logger.log(
                    &format!("Set ability {} var {:?} delta {}", ability, var, delta),
                    &LogContext::Effect,
                );

                let prev_value = ExpressionInt::AbilityVar {
                    house: *house,
                    ability: ability.clone(),
                    var: *var,
                }
                .calculate(&context, world, resources)?;
                let faction = Faction::from_entity(context.owner, world, &resources);
                TeamPool::set_ability_var_int(
                    house,
                    ability,
                    var,
                    prev_value + delta,
                    &faction,
                    resources,
                );
            }
            Effect::If {
                condition,
                then,
                r#else,
            } => {
                if condition.calculate(&context, world, resources)? {
                    resources
                        .action_queue
                        .push_front(Action::new(context.clone(), then.deref().clone()));
                } else if let Some(r#else) = r#else {
                    resources
                        .action_queue
                        .push_front(Action::new(context.clone(), r#else.deref().clone()));
                }
            }
            Effect::ShowText { text, color } => {
                let position = context.vars.get_vec2(&VarName::Position);
                let color = color
                    .or_else(|| {
                        context
                            .vars
                            .try_get_color(&VarName::Color)
                            .or_else(|| Some(context.vars.get_color(&VarName::HouseColor1)))
                    })
                    .unwrap();
                node.add_effect(VfxSystem::vfx_show_text(
                    resources,
                    &text,
                    Rgba::WHITE,
                    color,
                    position,
                    1,
                    0.0,
                ))
            }
            Effect::ShowCurve { color } => {
                let color = color
                    .or_else(|| {
                        context
                            .vars
                            .try_get_color(&VarName::Color)
                            .or_else(|| Some(context.vars.get_color(&VarName::HouseColor1)))
                    })
                    .unwrap();
                let from = ContextSystem::try_get_position(context.owner, world)
                    .context("Failed to get owner")?;
                let to = ContextSystem::try_get_position(context.target, world)
                    .context("Failed to get target")?;
                node.add_effect(VfxSystem::vfx_show_curve(resources, from, to, color));
            }
            Effect::Kill => {
                let mut entry = world.entry_mut(context.target).unwrap();
                let health = entry.get_component_mut::<HealthComponent>().unwrap();
                health.damage = i32::MAX as usize;
            }
            Effect::Revive { slot } => {
                let slot = slot
                    .as_ref()
                    .and_then(|x| Some(x.calculate(&context, world, resources).ok()?))
                    .unwrap_or_default() as usize;
                let (mut corpse, faction) = resources
                    .unit_corpses
                    .remove(&context.target)
                    .context("Target is not a corpse")?;
                corpse.health = 1;
                context.target = corpse.unpack(world, resources, slot, faction, None);
            }
            Effect::Aoe { factions, effect } => {
                UnitSystem::collect_factions(world, &HashSet::from_iter(factions.clone()))
                    .iter()
                    .for_each(|(entity, _)| {
                        resources.action_queue.push_front(Action::new(
                            Context {
                                target: *entity,
                                ..context.clone()
                            },
                            effect.deref().clone(),
                        ));
                    })
            }
            Effect::TakeVar {
                var,
                entity,
                new_name,
                effect,
            } => resources.action_queue.push_front(Action::new(
                {
                    let mut vars = context.vars.clone();
                    vars.insert(
                        new_name.unwrap_or(*var),
                        ContextSystem::get_context(
                            entity.calculate(&context, world, resources)?,
                            world,
                        )
                        .vars
                        .get(&var)
                        .clone(),
                    );
                    Context { vars, ..context }
                },
                effect.deref().clone(),
            )),
            Effect::RemoveTrigger => {
                if let Some(mut entry) = world.entry(context.target) {
                    entry.remove_component::<Trigger>();
                }
            }
            Effect::SetFaction { faction } => {
                let faction = faction.calculate(&context, world, resources)?;
                let mut target = world
                    .entry(context.target)
                    .context("Failed to get target")?;
                resources.logger.log(
                    &format!(
                        "{:?} Faction {:?} -> {:?}",
                        context.target,
                        target.get_component::<UnitComponent>().unwrap().faction,
                        faction
                    ),
                    &LogContext::Effect,
                );
                target.get_component_mut::<UnitComponent>()?.faction = faction;
            }
            Effect::SetSlot { slot } => {
                let slot = slot.calculate(&context, world, resources)? as usize;
                let mut target = world
                    .entry(context.target)
                    .context("Failed to get target")?;
                target.get_component_mut::<UnitComponent>()?.slot = slot;
            }
            Effect::FindTarget {
                faction,
                condition,
                effect,
            } => {
                let faction = faction.calculate(&context, world, resources)?;
                let target = UnitSystem::collect_faction(world, faction)
                    .into_iter()
                    .find(|(entity, _)| {
                        if let Some(context) = ContextSystem::try_get_context(*entity, world).ok() {
                            match condition.calculate(&context, world, resources) {
                                Ok(value) => value,
                                Err(_) => false,
                            }
                        } else {
                            false
                        }
                    });
                if let Some((target, _)) = target {
                    context.target = target;
                    resources
                        .action_queue
                        .push_front(Action::new(context.clone(), effect.deref().clone()));
                }
            }
        }
        Ok(match self.after.as_deref() {
            Some(after) => Self::process(after, context, world, resources)?.merge(&node),
            None => node,
        })
    }
}
