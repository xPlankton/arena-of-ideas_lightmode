use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AoeEffect {
    pub filter: TargetFilter,
    #[serde(default)]
    pub skip_current_target: bool,
    pub range: Option<Coord>,
    pub effect: Effect,
}

impl EffectContainer for AoeEffect {
    fn walk_effects_mut(&mut self, f: &mut dyn FnMut(&mut Effect)) {
        self.effect.walk_mut(f);
    }
}

impl EffectImpl for AoeEffect {
    fn process(self: Box<Self>, context: EffectContext, logic: &mut logic::Logic) {
        let effect = *self;
        let caster = context
            .caster
            .and_then(|id| {
                logic
                    .model
                    .units
                    .get(&id)
                    .or(logic.model.dead_units.get(&id))
            })
            .expect("Caster not found");
        let caster_faction = caster.faction;
        let center = context
            .target
            .and_then(|id| logic.model.units.get(&id).map(|unit| unit.position))
            .expect("Target not found");
        logic
            .model
            .render_model
            .add_text(center, "AOE", Rgba::RED, crate::render::TextType::Aoe);
        for unit in &logic.model.units {
            if effect.skip_current_target && Some(unit.id) == context.target {
                continue;
            }
            if let Some(range) = effect.range {
                if unit.position.distance(&center) > range {
                    continue;
                }
            }
            if !effect.filter.matches(unit.faction, caster_faction) {
                continue;
            }
            logic.effects.push_front(QueuedEffect {
                effect: effect.effect.clone(),
                context: EffectContext {
                    from: context.target,
                    target: Some(unit.id),
                    ..context.clone()
                },
            });
        }
    }
}
