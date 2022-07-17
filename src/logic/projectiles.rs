use super::*;

impl Logic<'_> {
    pub fn process_projectiles(&mut self) {
        let mut delete_projectiles = Vec::new();
        for id in self.model.projectiles.ids().copied().collect::<Vec<Id>>() {
            let mut projectile = self.model.projectiles.remove(&id).unwrap();

            let mut caster = self.model.units.remove(&projectile.caster);
            let max_distance = projectile.speed * self.delta_time;
            if let Some(mut target) = self.model.units.remove(&projectile.target) {
                projectile.target_position = target.position.to_world();
                if (projectile.position - projectile.target_position).len() < max_distance {
                    self.effects.push_back(QueuedEffect {
                        effect: projectile.effect.clone(),
                        context: EffectContext {
                            caster: Some(projectile.caster),
                            from: Some(projectile.caster),
                            target: Some(target.id),
                            vars: projectile.vars.clone(),
                            status_id: None,
                        },
                    });
                    delete_projectiles.push(projectile.id);
                }
                self.model.units.insert(target);
            }
            if let Some(caster) = caster {
                self.model.units.insert(caster);
            }
            let distance = (projectile.target_position - projectile.position).len();
            if distance < max_distance {
                delete_projectiles.push(projectile.id);
            }
            projectile.position += (projectile.target_position - projectile.position)
                .clamp_len(..=projectile.speed * self.delta_time);

            self.model.projectiles.insert(projectile);
        }
        for id in delete_projectiles {
            self.model.projectiles.remove(&id);
        }
    }
}
