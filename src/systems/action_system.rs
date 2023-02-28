use super::*;

pub struct ActionSystem {}

impl ActionSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run_ticks(world: &mut legion::World, resources: &mut Resources) {
        let mut ticks = 0;
        while Self::tick(world, resources) && ticks < 1000 {
            ticks += 1;
        }
    }

    pub fn tick(world: &mut legion::World, resources: &mut Resources) -> bool {
        StatusPool::init_new_statuses(world, resources);
        let Some(action) = resources.action_queue.pop_front() else { return false };
        debug!(
            "Procession action: {:?} context: {:?}",
            action.effect, action.context
        );
        match action
            .effect
            .process(action.context.clone(), world, resources)
        {
            Ok(_) => {}
            Err(error) => error!("Effect process error: {}", error),
        }
        ContextSystem::refresh_all(world);
        true
    }
}

impl System for ActionSystem {
    fn update(&mut self, world: &mut legion::World, resources: &mut Resources) {
        Self::tick(world, resources);
    }
}

pub struct Action {
    pub context: Context,
    pub effect: Effect,
}

impl Action {
    pub fn new(context: Context, effect: Effect) -> Self {
        Self { context, effect }
    }
}
