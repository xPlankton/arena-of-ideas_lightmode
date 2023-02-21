use super::*;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Condition {
    EqualsInt {
        a: ExpressionInt,
        b: ExpressionInt,
    },
    LessInt {
        a: ExpressionInt,
        b: ExpressionInt,
    },
    MoreInt {
        a: ExpressionInt,
        b: ExpressionInt,
    },
    SlotOccupied {
        slot: ExpressionInt,
        faction: Faction,
    },
}

impl Condition {
    pub fn calculate(
        &self,
        context: &Context,
        world: &legion::World,
        resources: &Resources,
    ) -> bool {
        debug!("Calculating condition {:?} {:?}", self, context);
        match self {
            Condition::EqualsInt { a, b } => {
                a.calculate(context, world, resources) == b.calculate(context, world, resources)
            }
            Condition::LessInt { a, b } => {
                a.calculate(context, world, resources) < b.calculate(context, world, resources)
            }
            Condition::MoreInt { a, b } => {
                a.calculate(context, world, resources) > b.calculate(context, world, resources)
            }
            Condition::SlotOccupied { slot, faction } => SlotSystem::find_unit_by_slot(
                slot.calculate(context, world, resources) as usize,
                faction,
                world,
            )
            .is_some(),
        }
    }
}
