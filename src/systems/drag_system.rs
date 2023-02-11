use super::*;

pub struct DragSystem {
    dragged: Option<legion::Entity>,
}

impl DragSystem {
    pub fn new() -> Self {
        Self { dragged: None }
    }

    fn get_hovered_unit(world: &legion::World, resources: &Resources) -> Option<legion::Entity> {
        <(&Position, &EntityComponent, &UnitComponent)>::query()
            .iter(world)
            .find_map(|(position, entity, _)| {
                if (resources.mouse_pos - position.0).len() < UNIT_RADIUS {
                    Some(entity.entity)
                } else {
                    None
                }
            })
    }
}

impl System for DragSystem {
    fn update(&mut self, world: &mut legion::World, resources: &mut Resources) {
        if resources
            .down_mouse_buttons
            .contains(&geng::MouseButton::Left)
        {
            if let Some(dragged) = Self::get_hovered_unit(world, resources) {
                self.dragged = Some(dragged);
            }
        }
        if self.dragged.is_some()
            && !resources
                .pressed_mouse_buttons
                .contains(&geng::MouseButton::Left)
        {
            self.dragged = None;
        }
        if let Some(dragged) = self.dragged {
            world
                .entry(dragged)
                .unwrap()
                .get_component_mut::<Position>()
                .unwrap()
                .0 = resources.mouse_pos;
        }
    }
}
