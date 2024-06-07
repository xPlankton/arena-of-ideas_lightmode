use super::*;

#[derive(Asset, Deserialize, Serialize, TypePath, Debug, Clone, Default)]
pub struct PackedTeam {
    pub units: Vec<PackedUnit>,
}

impl PackedTeam {
    pub fn unpack(self, faction: Faction, world: &mut World) {
        for (slot, unit) in self.units.into_iter().enumerate() {
            unit.unpack(
                TeamPlugin::entity(faction, world),
                Some(slot as i32 + 1),
                world,
            );
        }
    }
}
