use super::*;

#[derive(Deserialize, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "028620be-3b01-4e20-b62e-a631f0db4777"]
pub struct PackedUnit {
    pub hp: i32,
    pub atk: i32,
    pub house: String,
    #[serde(default)]
    pub trigger: Trigger,
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub representation: Representation,
    #[serde(default)]
    pub state: VarState,
    #[serde(default)]
    pub statuses: Vec<(String, i32)>,
}

const LOCAL_TRIGGER: &str = "_local";

impl PackedUnit {
    pub fn unpack(mut self, parent: Entity, slot: Option<usize>, world: &mut World) -> Entity {
        debug!("Unpack unit {:?}", &self);
        let entity = Options::get_unit_rep(world)
            .clone()
            .unpack(None, Some(parent), world);
        world
            .entity_mut(entity)
            .insert(PickableBundle::default())
            .insert(RaycastPickTarget::default())
            .insert(On::<Pointer<Over>>::run(UnitPlugin::hover_unit))
            .insert(On::<Pointer<Out>>::run(UnitPlugin::unhover_unit));
        {
            let entity = self.representation.unpack(None, Some(entity), world);
            world.entity_mut(entity).insert(UnitRepresentation);
        }
        self.state
            .init(VarName::Hp, VarValue::Int(self.hp))
            .init(VarName::Atk, VarValue::Int(self.atk))
            .init(VarName::House, VarValue::String(self.house.clone()))
            .init(VarName::Name, VarValue::String(self.name.clone()))
            .init(VarName::Position, VarValue::Vec2(default()))
            .init(
                VarName::Slot,
                VarValue::Int(slot.unwrap_or_default() as i32),
            )
            .init(
                VarName::Description,
                VarValue::String(self.description.to_owned()),
            );
        world
            .entity_mut(entity)
            .insert((Unit, Name::new(self.name), self.state));
        Status::spawn(LOCAL_TRIGGER.to_owned(), self.trigger, world)
            .insert(VarState::default())
            .set_parent(entity);
        for (status, charges) in self.statuses {
            Status::change_charges(&status, entity, charges, world).unwrap();
        }
        entity
    }

    pub fn pack(entity: Entity, world: &World) -> Self {
        let rep_entity = *world
            .get::<Children>(entity)
            .unwrap()
            .into_iter()
            .find(|x| world.get::<UnitRepresentation>(**x).is_some())
            .unwrap();
        let representation = Representation::pack(rep_entity, world);
        let state = VarState::get(entity, world).clone();
        let hp = state.get_int(VarName::Hp).unwrap();
        let atk = state.get_int(VarName::Atk).unwrap();
        let name = state.get_string(VarName::Name).unwrap();
        let description = state.get_string(VarName::Description).unwrap();
        let house = state.get_string(VarName::House).unwrap();
        let mut trigger = None;
        let mut statuses = Vec::default();
        for entity in Status::collect_entity_statuses(entity, world) {
            let status = world.get::<Status>(entity).unwrap();
            if status.name.eq(LOCAL_TRIGGER) {
                trigger = Some(status.trigger.clone());
            } else {
                statuses.push((
                    status.name.to_owned(),
                    VarState::get(entity, world)
                        .get_int(VarName::Charges)
                        .unwrap(),
                ));
            }
        }
        let trigger = trigger.unwrap();

        Self {
            hp,
            atk,
            house,
            name,
            trigger,
            representation,
            state,
            description,
            statuses,
        }
    }
}
