use super::*;

impl StrengthModifier {
    pub fn apply(&self, effect: &mut Effect) {
        effect.walk_mut(&mut |effect| match effect {
            Effect::Damage(damage) => {
                damage.value = Expr::Sum {
                    a: Box::new(Expr::Mul {
                        a: Box::new(damage.value.clone()),
                        b: Box::new(Expr::Const {
                            value: self.multiplier,
                        }),
                    }),
                    b: Box::new(Expr::Const { value: self.add }),
                }
            }
            _ => {}
        });
    }
}

impl Effect {
    pub fn apply_modifier(&mut self, modifier: &Modifier) {
        match modifier {
            Modifier::Strength(modifier) => modifier.apply(self),
        }
    }
}
