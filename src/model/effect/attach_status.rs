use crate::model::status::StatusAction;

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Who {
    Owner,
    Creator,
    Target,
}

impl fmt::Display for Who {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttachStatusEffect {
    pub status: StatusRef,
    #[serde(default)]
    pub vars: HashMap<VarName, i32>,
}

impl AttachStatusEffect {
    pub fn new(status: StatusRef, vars: HashMap<VarName, i32>) -> Self {
        Self { status, vars }
    }
}

impl EffectContainer for AttachStatusEffect {
    fn walk_effects_mut(&mut self, _f: &mut dyn FnMut(&mut Effect)) {}
}

impl EffectImpl for AttachStatusEffect {
    fn process(self: Box<Self>, context: EffectContext, logic: &mut logic::Logic) {
        let effect = *self;
        let status_name = effect.status.name();
        let target = logic.model.get_who(Who::Target, &context);
        // Check if unit is immune to status attachment
        if target
            .flags
            .iter()
            .any(|flag| matches!(flag, UnitStatFlag::AttachStatusImmune))
        {
            return;
        }
        let target_position = target.position.clone();
        let target = target.id;

        let mut status = effect.status.get(&logic.model.statuses).clone().attach(
            target,
            context.owner,
            logic.model.next_id,
        );
        if !status.status.hidden {
            logic.model.render_model.add_text(
                target_position,
                &format!("+{}", status_name),
                status.status.color,
                crate::render::TextType::Status,
            );
        }

        status.vars.extend(effect.vars.into_iter());
        if !status.is_inited {
            for trigger_effect in status
                .trigger()
                .filter(|effect| matches!(effect.trigger, StatusTrigger::Init))
            {
                let mut vars = trigger_effect.vars.clone();
                vars.extend(context.vars.clone());
                logic.effects.push_front(
                    EffectContext {
                        vars,
                        color: trigger_effect.status_color,
                        status_id: Some(trigger_effect.status_id),
                        ..context.clone()
                    },
                    trigger_effect.effect,
                )
            }
            status.is_inited = true;
        }

        let target = logic.model.get_who_mut(Who::Target, &context);
        let attached_status_id = unit_attach_status(status, &mut target.all_statuses);
        debug!(
            "Attach status {}#{} {}#{}",
            effect.status.name(),
            attached_status_id,
            target.unit_type,
            target.id
        );

        let target_id = target.id;
        logic.trigger_status_attach(context, attached_status_id, status_name);
    }
}

impl Logic {
    pub fn trigger_status_attach(
        &mut self,
        context: EffectContext,
        attached_status_id: Id,
        status_name: &StatusName,
    ) {
        let target = self.model.get_who(Who::Target, &context);
        for trigger_effect in target.trigger().filter(|effect| match &effect.trigger {
            StatusTrigger::SelfDetectAttach {
                status_name: detect,
                status_action,
            } => detect == status_name && status_action == &StatusAction::Add,
            _ => false,
        }) {
            let mut vars = trigger_effect.vars.clone();
            vars.extend(context.vars.clone());
            self.effects.push_back(
                EffectContext {
                    vars,
                    status_id: Some(attached_status_id),
                    color: trigger_effect.status_color,
                    ..context.clone()
                },
                trigger_effect.effect,
            );
        }

        for other in &self.model.units {
            for trigger_effect in other.trigger().filter(|effect| match &effect.trigger {
                StatusTrigger::DetectAttach {
                    status_name: detect,
                    filter,
                    status_action,
                } => {
                    other.id != target.id
                        && detect == status_name
                        && status_action == &StatusAction::Add
                        && filter.matches(target.faction, other.faction)
                }
                _ => false,
            }) {
                let mut vars = trigger_effect.vars.clone();
                vars.extend(context.vars.clone());
                self.effects.push_back(
                    EffectContext {
                        owner: other.id,
                        vars,
                        status_id: Some(attached_status_id),
                        color: trigger_effect.status_color,
                        ..context.clone()
                    },
                    trigger_effect.effect,
                );
            }
        }
    }
}
