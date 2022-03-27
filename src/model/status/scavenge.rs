use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScavengeStatus {
    pub who: TargetFilter,
    pub range: Coord,
    pub alliance: Option<Alliance>,
    pub effect: Effect,
}

impl EffectContainer for ScavengeStatus {
    fn walk_effects_mut(&mut self, f: &mut dyn FnMut(&mut Effect)) {
        self.effect.walk_mut(f);
    }
}

impl StatusImpl for ScavengeStatus {}
