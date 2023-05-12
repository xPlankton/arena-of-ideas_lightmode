use geng::prelude::itertools::Itertools;

use super::*;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct BonusEffect {
    pub effect: EffectWrapped,
    pub rarity: Rarity,
    pub description: String,
    #[serde(default)]
    pub single_target: bool,
    #[serde(skip)]
    pub target: Option<(legion::Entity, String)>,
}

impl BonusEffect {
    pub fn new_buff_effect(g: usize, rarity: Rarity, resources: &Resources) -> Self {
        let mut single_target = false;
        let (effect, description) = if rarity == Rarity::Legendary {
            let buff = BuffPool::random_team_buff(resources);
            let mut effects = Vec::default();
            for (name, charges) in buff.statuses.iter() {
                for _ in 0..*charges {
                    effects.push(
                        Effect::AddTeamStatus {
                            name: name.to_owned(),
                        }
                        .wrap(),
                    )
                }
            }
            let effect = Effect::List {
                effects: effects.into_iter().map(|x| Box::new(x)).collect_vec(),
            }
            .wrap();
            (effect, format!("Add Team status {}", buff.prefix))
        } else {
            let (name, mut charges) = BuffPool::random_unit_buff(resources);
            match rarity {
                Rarity::Common => {
                    single_target = true;
                    (
                        Effect::ChangeStatus {
                            name: name.to_owned(),
                            charges: ExpressionInt::Const { value: charges },
                        }
                        .wrap(),
                        format!("Add {} ({})", name.to_owned(), charges),
                    )
                }
                Rarity::Rare => {
                    single_target = true;
                    charges *= 3;
                    (
                        Effect::ChangeStatus {
                            name: name.to_owned(),
                            charges: ExpressionInt::Const { value: charges },
                        }
                        .wrap(),
                        format!("Add {} ({})", name.to_owned(), charges),
                    )
                }
                Rarity::Epic => {
                    let effect = Box::new(
                        Effect::ChangeStatus {
                            name: name.to_owned(),
                            charges: ExpressionInt::Const { value: charges },
                        }
                        .wrap(),
                    );
                    let effect = Effect::Aoe {
                        factions: vec![ExpressionFaction::Team],
                        effect,
                        exclude_self: false,
                    }
                    .wrap();
                    (
                        effect,
                        format!("Add {} ({}) to everyone", name.to_owned(), charges),
                    )
                }
                _ => panic!(),
            }
        };
        let (effect, description) = Self::add_g_effect(g, effect, description);
        Self {
            effect,
            rarity,
            description,
            single_target,
            target: None,
        }
    }

    pub fn new_slot_effect(g: usize, rarity: Rarity) -> Self {
        let value: i32 = match rarity {
            Rarity::Common | Rarity::Rare => 1,
            Rarity::Epic | Rarity::Legendary => 2,
        };
        let effect = Effect::ChangeTeamVarInt {
            var: VarName::Slots,
            delta: ExpressionInt::Const { value },
            faction: Some(ExpressionFaction::Team),
        }
        .wrap();
        let description = format!("+{value} slots");
        let (effect, description) = Self::add_g_effect(g, effect, description);
        Self {
            effect,
            rarity,
            description,
            single_target: default(),
            target: default(),
        }
    }

    fn add_g_effect(
        g: usize,
        effect: EffectWrapped,
        description: String,
    ) -> (EffectWrapped, String) {
        let description = format!("+{g} g, {description}");
        let g_effect = Box::new(
            Effect::ChangeTeamVarInt {
                var: VarName::G,
                delta: ExpressionInt::Const { value: g as i32 },
                faction: Some(ExpressionFaction::Team),
            }
            .wrap(),
        );
        (
            Effect::List {
                effects: vec![g_effect, Box::new(effect)],
            }
            .wrap(),
            description,
        )
    }
}

#[derive(
    Clone, Copy, Deserialize, Serialize, Debug, Eq, PartialEq, Hash, enum_iterator::Sequence,
)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

impl Rarity {
    pub fn weight(&self) -> i32 {
        match self {
            Rarity::Common => 100,
            Rarity::Rare => 25,
            Rarity::Epic => 10,
            Rarity::Legendary => 3,
        }
    }

    pub fn generate(&self, value: usize, units: usize, resources: &Resources) -> BonusEffect {
        let rng = &mut thread_rng();
        let mut g = value;
        match self {
            Rarity::Common => g += rng.gen_range(3..=4),
            Rarity::Rare => g += rng.gen_range(3..=7),
            Rarity::Epic => g += rng.gen_range(3..=10),
            Rarity::Legendary => g += rng.gen_range(3..=13),
        };
        match units > 0 && rng.gen_bool(0.7) {
            true => BonusEffect::new_buff_effect(g, *self, resources),
            false => BonusEffect::new_slot_effect(g, *self),
        }
    }
}
