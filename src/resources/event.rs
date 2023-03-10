use super::*;

#[derive(Debug)]
pub enum Event {
    Init { status: String, context: Context },
    ModifyIncomingDamage { context: Context },
    BeforeIncomingDamage { context: Context },
    AfterIncomingDamage { context: Context },
    BeforeDeath { context: Context },
    BattleOver,
    Buy { context: Context },
    Sell { context: Context },
    RemoveFromTeam { context: Context },
    AfterStrike { context: Context },
}

impl Event {
    pub fn send(&self, resources: &mut Resources, world: &legion::World) -> Option<Context> {
        resources
            .logger
            .log(&format!("Send event {:?}", self), &LogContext::Event);
        match self {
            Event::BeforeIncomingDamage { context }
            | Event::AfterIncomingDamage { context }
            | Event::BeforeDeath { context }
            | Event::Buy { context }
            | Event::Sell { context }
            | Event::RemoveFromTeam { context } => {
                Self::trigger(resources, context, &context.target, self);
                None
            }
            Event::AfterStrike { context } => {
                Self::trigger(resources, context, &context.owner, self);
                None
            }
            Event::Init { status, context } => {
                resources
                    .status_pool
                    .defined_statuses
                    .get(status)
                    .expect("Failed to find defined status for initialization")
                    .trigger
                    .catch_event(
                        self,
                        &mut resources.action_queue,
                        context.clone(),
                        &resources.logger,
                    );
                None
            }
            Event::BattleOver => {
                resources
                    .status_pool
                    .active_statuses
                    .values()
                    .map(|map| map.iter())
                    .flatten()
                    .map(|(status_name, status_context)| {
                        (
                            &resources
                                .status_pool
                                .defined_statuses
                                .get(status_name)
                                .expect("Failed to find defined status")
                                .trigger,
                            status_context,
                        )
                    })
                    .for_each(|(trigger, status_context)| {
                        trigger.catch_event(
                            self,
                            &mut resources.action_queue,
                            {
                                let context = {
                                    let context = status_context.clone();
                                    context.merge(
                                        &ContextSystem::get_context(status_context.owner, world),
                                        true,
                                    )
                                };
                                context
                            },
                            &resources.logger,
                        )
                    });
                None
            }
            Event::ModifyIncomingDamage { context } => {
                let mut context = context.clone();
                let mut damage = context.vars.get_int(&VarName::Damage);
                resources
                    .status_pool
                    .collect_triggers(&context.target)
                    .iter()
                    .for_each(|(trigger, status_context)| match trigger {
                        Trigger::ModifyIncomingDamage { value } => {
                            damage = match value.calculate(
                                &context.merge(status_context, false),
                                world,
                                resources,
                            ) {
                                Ok(value) => value,
                                Err(_) => damage,
                            };
                            context.vars.insert(VarName::Damage, Var::Int(damage));
                        }
                        _ => {}
                    });
                Some(context)
            }
        }
    }

    fn trigger(
        resources: &mut Resources,
        context: &Context,
        entity: &legion::Entity,
        event: &Event,
    ) {
        resources
            .status_pool
            .collect_triggers(entity)
            .iter()
            .for_each(|(trigger, status_context)| {
                trigger.catch_event(
                    event,
                    &mut resources.action_queue,
                    context.merge(&status_context, false),
                    &resources.logger,
                )
            });
    }
}
