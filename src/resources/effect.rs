use super::*;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, AsRefStr, EnumIter)]
pub enum Effect {
    #[default]
    Noop,
    Damage,
    ChangeStatus(String),
    UseAbility(String, i32),
    WithTarget(Expression, Box<Effect>),
    WithVar(VarName, Expression, Box<Effect>),
}

impl Effect {
    pub fn invoke(&self, context: &mut Context, world: &mut World) -> Result<()> {
        debug!("Processing {:?}\n{:?}", self, context);
        let owner = context.owner();
        match self {
            Effect::Noop => {}
            Effect::Damage => {
                let target = context.get_target()?;
                let value = context
                    .get_var(VarName::Damage, world)
                    .unwrap_or(context.get_var(VarName::Pwr, world)?)
                    .get_int()?;
                if value > 0 {
                    debug!("deal {value} dmg to {target:?}");
                    VarState::get_mut(target, world).change_int(VarName::Dmg, value);
                }
            }
            Effect::ChangeStatus(name) => {
                let delta = context
                    .get_var(VarName::Charges, world)
                    .unwrap_or(VarValue::Int(1))
                    .get_int()?;
                Status::change_charges(&name, context.get_target()?, delta, world);
            }
            Effect::UseAbility(name, base) => {
                let ability = GameAssets::get(world)
                    .abilities
                    .get(name)
                    .with_context(|| format!("Ability not found {name}"))?;
                let charges = context
                    .get_var(VarName::Level, world)
                    .map(|v| v.get_int().unwrap())
                    .unwrap_or(1)
                    + *base;
                let caster = owner;
                let context = context
                    .clone()
                    .set_var(VarName::Charges, VarValue::Int(charges))
                    .set_caster(caster)
                    .take();
                ActionPlugin::action_push_front(ability.effect.clone(), context, world);
            }
            Effect::WithTarget(target, effect) => {
                let target = target.get_value(context, world)?;
                let targets = target
                    .get_entity_list()?
                    .into_iter()
                    .sorted_by_key(|e| -VarState::get(*e, world).get_int(VarName::Slot).unwrap())
                    .collect_vec();
                let delay = 0.2;
                for target in targets {
                    let context = context.set_target(target).clone();
                    ActionPlugin::action_push_front_with_delay(
                        effect.deref().clone(),
                        context,
                        delay,
                        world,
                    );
                }
            }
            Effect::WithVar(var, value, effect) => {
                let context = context
                    .set_var(*var, value.get_value(context, world)?)
                    .clone();
                ActionPlugin::action_push_front(effect.deref().clone(), context, world);
            }
        }
        Ok(())
    }
}
