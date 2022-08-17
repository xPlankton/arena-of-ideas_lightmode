use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ReviveEffect {
    pub health: Expr,
}

impl EffectContainer for ReviveEffect {
    fn walk_effects_mut(&mut self, _f: &mut dyn FnMut(&mut Effect)) {}
}

impl EffectImpl for ReviveEffect {
    fn process(self: Box<Self>, context: EffectContext, logic: &mut logic::Logic) {
        let effect = *self;
        let health = effect.health.calculate(&context, logic);
        let mut unit = context
            .target
            .and_then(|id| logic.model.units.get_mut(&id))
            .expect("Target not found");
        unit.stats.health = health;
        unit.permanent_stats.health = health;
        unit.is_dead = false;
    }
}
