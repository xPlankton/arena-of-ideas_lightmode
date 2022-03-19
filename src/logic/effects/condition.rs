use super::*;

impl Logic<'_> {
    pub fn check_condition(&self, condition: &Condition, context: &EffectContext) -> bool {
        match condition {
            Condition::Always => true,
            Condition::UnitHasStatus { who, status_type } => {
                let who = context.get(*who);
                let who = who
                    .and_then(|id| self.model.units.get(&id))
                    .expect("Caster or Target not found");
                who.all_statuses
                    .iter()
                    .any(|status| status.r#type() == *status_type)
            }
        }
    }
}
