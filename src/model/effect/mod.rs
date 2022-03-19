use super::*;

mod add_targets;
mod aoe;
mod attach_status;
mod chain;
mod change_context;
mod damage;
mod heal;
mod if_effect;
mod maybe_modify;
mod projectile;
mod spawn;
mod suicide;
mod time_bomb;

pub use add_targets::*;
pub use aoe::*;
pub use attach_status::*;
pub use chain::*;
pub use change_context::*;
pub use damage::*;
pub use heal::*;
pub use if_effect::*;
pub use maybe_modify::*;
pub use projectile::*;
pub use spawn::*;
pub use suicide::*;
pub use time_bomb::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum Effect {
    Noop,
    InstantAction,
    Projectile(Box<ProjectileEffect>),
    Damage(Box<DamageEffect>),
    AttachStatus(Box<AttachStatusEffect>),
    Spawn(Box<SpawnEffect>),
    AOE(Box<AoeEffect>),
    TimeBomb(Box<TimeBombEffect>),
    Suicide(Box<SuicideEffect>),
    Chain(Box<ChainEffect>),
    AddTargets(Box<AddTargetsEffect>),
    Repeat { times: usize, effect: Box<Effect> },
    Random { choices: Vec<WeighedEffect> },
    List { effects: Vec<Effect> },
    If(Box<IfEffect>),
    MaybeModify(Box<MaybeModifyEffect>),
    ChangeContext(Box<ChangeContextEffect>),
    Heal(Box<HealEffect>),
}
