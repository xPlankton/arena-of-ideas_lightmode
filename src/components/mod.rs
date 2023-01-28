use super::*;

mod attack;
mod context;
mod flags;
mod game_state;
mod hp;
mod position;
mod shader;
mod unit;
mod vars;

pub use attack::*;
pub use context::*;
pub use flags::*;
pub use game_state::*;
pub use hp::*;
pub use position::*;
pub use shader::*;
pub use unit::*;
pub use vars::*;

/// Components that can be deserialized from json
#[derive(Deserialize, Debug)]
#[serde(tag = "component")]
pub enum Component {
    Hp { max: Hp },
    Attack { value: Hp },
    StatusContainer { statuses: Vec<Status> },
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Component {
    pub fn add_to_entry(
        &self,
        entry: &mut legion::world::Entry,
        entity: &legion::Entity,
        all_statuses: &mut Statuses,
    ) {
        match self {
            Component::Hp { max } => entry.add_component(HpComponent::new(*max)),
            Component::Attack { value } => entry.add_component(AttackComponent::new(*value)),
            Component::StatusContainer { statuses } => {
                let mut entity_statuses = all_statuses
                    .active_statuses
                    .remove(entity)
                    .unwrap_or_default();
                let context = Context {
                    owner: entity.clone(),
                    target: entity.clone(),
                    creator: entity.clone(),
                };
                for status in statuses.into_iter() {
                    all_statuses
                        .defined_statuses
                        .insert(status.name.clone(), status.clone());
                    entity_statuses.insert(status.name.clone(), context.clone());
                }
                all_statuses
                    .active_statuses
                    .insert(entity.clone(), entity_statuses);
            }
        }
    }
}
