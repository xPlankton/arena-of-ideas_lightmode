use super::*;

mod ai;
mod template;

pub use ai::*;
pub use template::*;

pub type UnitType = String;
pub type Tier = u32;

#[derive(Serialize, Deserialize, Clone)]
pub enum ActionState {
    None,
    Start { time: Time, target: Id },
    Cooldown { time: Time },
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ActionProperties {
    pub cooldown: Time,
    pub animation_delay: Time,
    pub range: Coord,
    #[serde(default)]
    pub effect: Effect,
}

#[derive(Serialize, Deserialize, HasId, Clone)]
pub struct Unit {
    pub id: Id,
    pub unit_type: UnitType,
    pub spawn_animation_time_left: Option<Time>,
    pub attached_statuses: Vec<AttachedStatus>,
    pub all_statuses: Vec<Status>,
    pub faction: Faction,
    pub action_state: ActionState,
    pub health: Health,
    pub max_hp: Health,
    pub base_damage: Health,
    pub crit_chance: R32,
    pub face_dir: Vec2<Coord>,
    pub position: Vec2<Coord>,
    pub speed: Coord,
    pub action: ActionProperties,
    pub radius: Coord,
    pub move_ai: MoveAi,
    pub target_ai: TargetAi,
    pub ability_cooldown: Option<Time>,
    pub clans: HashSet<Clan>,
    pub next_action_modifiers: Vec<Modifier>,
    #[serde(skip)]
    pub render: RenderConfig,
    pub last_action_time: Time,
    pub last_injure_time: Time,
    pub random_number: R32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitStat {
    MaxHealth,
    Radius,
    BaseDamage,
    CritChance,
    Speed,
}

impl Unit {
    pub fn new(
        template: &UnitTemplate,
        id: Id,
        unit_type: UnitType,
        faction: Faction,
        position: Vec2<Coord>,
    ) -> Self {
        Self {
            id,
            unit_type,
            spawn_animation_time_left: Some(template.spawn_animation_time),
            attached_statuses: template
                .statuses
                .iter()
                .map(|status| AttachedStatus {
                    status: status.clone(),
                    caster: None,
                    time: None,
                    duration: None,
                })
                .collect(),
            all_statuses: Vec::new(),
            faction,
            action_state: ActionState::None,
            health: template.health,
            max_hp: template.health,
            base_damage: template.base_damage,
            crit_chance: template.crit_chance,
            face_dir: Vec2::ZERO,
            position,
            speed: template.speed,
            radius: template.radius,
            action: template.action.clone(),
            move_ai: template.move_ai,
            target_ai: template.target_ai,
            render: template.render_config.clone(),
            next_action_modifiers: Vec::new(),
            ability_cooldown: None,
            clans: template.clans.clone(),
            last_action_time: Time::new(0.0),
            last_injure_time: Time::new(0.0),
            random_number: r32(global_rng().gen_range(0.0..=1.0)),
        }
    }
    pub fn stat(&self, stat: UnitStat) -> R32 {
        match stat {
            UnitStat::MaxHealth => self.max_hp,
            UnitStat::Radius => self.radius,
            UnitStat::BaseDamage => self.base_damage,
            UnitStat::CritChance => self.crit_chance,
            UnitStat::Speed => self.speed,
        }
    }
    pub fn stat_mut(&mut self, stat: UnitStat) -> &mut R32 {
        match stat {
            UnitStat::MaxHealth => &mut self.max_hp,
            UnitStat::Radius => &mut self.radius,
            UnitStat::BaseDamage => &mut self.base_damage,
            UnitStat::CritChance => &mut self.crit_chance,
            UnitStat::Speed => &mut self.speed,
        }
    }
}
