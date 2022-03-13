use super::*;

impl Logic<'_> {
    pub fn process_time_bombs(&mut self) {
        for bomb in &mut self.model.time_bombs {
            bomb.time -= self.delta_time;
            if bomb.time <= Time::ZERO {
                self.effects.push_back(QueuedEffect {
                    effect: bomb.effect.clone(),
                    caster: bomb.caster,
                    target: Some(bomb.id),
                });
                self.model.dead_time_bombs.insert(bomb.clone());
            }
        }
        self.model.time_bombs.retain(|bomb| bomb.time > Time::ZERO);
    }
}
