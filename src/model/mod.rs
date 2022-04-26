use super::*;

mod ability;
mod alliances;
mod condition;
mod effect;
mod expr;
mod factions;
mod modifier;
mod particle;
mod projectile;
mod render;
mod status;
mod time_bomb;
mod unit;

pub use ability::*;
pub use alliances::*;
pub use condition::*;
pub use effect::*;
pub use expr::*;
pub use factions::*;
pub use modifier::*;
pub use particle::*;
pub use projectile::*;
pub use render::*;
pub use status::*;
pub use time_bomb::*;
pub use unit::*;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TargetFilter {
    All,
    Allies,
    Enemies,
}

impl TargetFilter {
    pub fn matches(&self, a: Faction, b: Faction) -> bool {
        match self {
            Self::Allies => a == b,
            Self::Enemies => a != b,
            Self::All => true,
        }
    }
}

#[derive(Clone)]
pub struct Model {
    pub next_id: Id,
    pub time: Time,
    pub units: Collection<Unit>,
    pub spawning_units: Collection<Unit>,
    pub dead_units: Collection<Unit>,
    pub projectiles: Collection<Projectile>,
    pub time_bombs: Collection<TimeBomb>,
    pub dead_time_bombs: Collection<TimeBomb>,
    pub particles: Collection<Particle>,
    pub config: Config,
    pub free_revives: usize,
    pub unit_templates: UnitTemplates,
    pub delayed_effects: std::collections::BinaryHeap<QueuedEffect<DelayedEffect>>,
}

impl Model {
    pub fn new(config: Config, unit_templates: UnitTemplates) -> Self {
        Self {
            next_id: 0,
            time: Time::ZERO,
            units: Collection::new(),
            spawning_units: Collection::new(),
            dead_units: Collection::new(),
            projectiles: Collection::new(),
            time_bombs: Collection::new(),
            dead_time_bombs: Collection::new(),
            particles: Collection::new(),
            config,
            free_revives: 0,
            unit_templates,
            delayed_effects: default(),
        }
    }
}
