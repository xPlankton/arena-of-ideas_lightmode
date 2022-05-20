use geng::prelude::itertools::Itertools;

use super::*;

mod archers;
mod assassins;
mod chainers;
mod charmers;
mod critters;
mod exploders;
mod freezers;
mod healers;
mod spawners;
mod splashers;
mod vampires;
mod warriors;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub enum Alliance {
    Spawners,
    Assassins,
    Critters,
    Archers,
    Freezers,
    Warriors,
    Healers,
    Vampires,
    Exploders,
    Splashers,
    Chainers,
    Charmers,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct AllianceEffect {
    /// Number of heroes required to activate the effect
    activate: usize,
    /// Whether effects with lower requirements should be removed
    #[serde(default)]
    replace: bool,
    /// Filter target units by factions
    #[serde(default)]
    factions: Option<Vec<Faction>>,
    /// Filter target units by clan
    #[serde(default)]
    clans: Option<Vec<Alliance>>,
    /// Statuses to apply to every target unit
    statuses: Vec<Status>,
}

impl AllianceEffect {
    /// Checks the filters (factions and clans) and applies the
    /// effects if the constraints are met.
    fn apply(&self, unit: &mut Unit) {
        if !self
            .factions
            .as_ref()
            .map(|factions| factions.contains(&unit.faction))
            .unwrap_or(true)
            || !self
                .clans
                .as_ref()
                .map(|clans| clans.iter().any(|clan| unit.alliances.contains(clan)))
                .unwrap_or(true)
        {
            return;
        }
        unit.attached_statuses
            .extend(self.statuses.iter().map(|status| AttachedStatus {
                status: status.clone(),
                caster: None,
                time: None,
                duration: None,
            }));
    }
}

impl Alliance {
    pub fn apply_effects(&self, unit: &mut Unit, effects: &AllianceEffects, party_members: usize) {
        let effects = match effects.get(self) {
            Some(effects) => effects,
            None => {
                error!("Failed to find effects for the alliance {self:?}");
                return;
            }
        };
        let effects = effects
            .iter()
            .filter(|effect| effect.activate <= party_members)
            .sorted_by_key(|effect| effect.activate);
        for effect in effects {
            effect.apply(unit);
            if effect.replace {
                break;
            }
        }
    }
}
