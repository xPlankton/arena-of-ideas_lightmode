use super::*;

pub struct GameStateSystem {
    pub systems: HashMap<GameState, Vec<Box<dyn System>>>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum GameState {
    MainMenu,
    Shop,
    Battle,
    Gallery,
    GameOver,
    CustomGame,
}

impl System for GameStateSystem {
    fn update(&mut self, world: &mut legion::World, resources: &mut Resources) {
        match resources.current_state {
            GameState::MainMenu => {
                if resources.options.custom_game.enable {
                    resources.transition_state = GameState::CustomGame;
                } else {
                    resources.transition_state = GameState::Shop;
                }
            }
            GameState::Battle => {
                if resources.input.down_keys.contains(&R) {
                    resources.tape_player.head = 0.0;
                }
                if resources.tape_player.tape.length() < resources.tape_player.head
                    || resources.input.down_keys.contains(&Escape)
                {
                    BattleSystem::finish_floor_battle(world, resources);
                }
            }
            GameState::Shop => {
                if resources.input.down_keys.contains(&Space) {
                    ShopSystem::switch_to_battle(world, resources);
                    resources.transition_state = GameState::Battle;
                }

                if resources.input.down_keys.contains(&G) {
                    resources.transition_state = GameState::Gallery;
                }

                if resources.input.down_keys.contains(&O) {
                    resources.transition_state = GameState::GameOver;
                }
                if resources.input.down_keys.contains(&R) {
                    resources.transition_state = GameState::MainMenu;
                }
                if resources.input.down_keys.contains(&C) {
                    ShopSystem::change_g(resources, 100);
                }
                if resources.input.down_keys.contains(&L) {
                    SaveSystem::load(world, resources);
                }
                if resources.input.down_keys.contains(&S) {
                    SaveSystem::save(world, resources);
                }
                if resources.input.down_keys.contains(&P) {
                    if let Some(entity) =
                        SlotSystem::find_unit_by_slot(1, &Faction::Shop, world, resources)
                    {
                        UnitSystem::delete_unit(entity, world, resources);
                    }
                    resources.hero_pool.list_top().clone().unpack(
                        world,
                        resources,
                        1,
                        Faction::Shop,
                        Some(SlotSystem::get_position(1, &Faction::Shop, resources)),
                    );
                }
            }
            GameState::Gallery => {
                if resources.input.down_keys.contains(&G) {
                    resources.transition_state = GameState::Shop;
                }
            }
            GameState::GameOver => {
                if resources.input.down_keys.contains(&Enter) {
                    resources.transition_state = GameState::Shop;
                }
            }
            GameState::CustomGame => {
                if resources.input.down_keys.contains(&R) {
                    resources.transition_state = GameState::MainMenu;
                    resources.tape_player.head = 0.0;
                }
            }
        }
        self.systems
            .get_mut(&resources.current_state)
            .and_then(|systems| {
                Some(
                    systems
                        .iter_mut()
                        .for_each(|system| system.update(world, resources)),
                )
            });

        self.transition(world, resources);
    }

    fn draw(
        &self,
        world: &legion::World,
        resources: &mut Resources,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        self.systems
            .get(&resources.current_state)
            .and_then(|systems| {
                Some(
                    systems
                        .iter()
                        .for_each(|system| system.draw(world, resources, framebuffer)),
                )
            });
    }

    fn ui<'a>(
        &'a mut self,
        cx: &'a ui::Controller,
        world: &'a legion::World,
        resources: &'a Resources,
    ) -> Box<dyn ui::Widget + 'a> {
        if let Some(widgets) = self
            .systems
            .get_mut(&resources.current_state)
            .and_then(|systems| {
                Some(
                    systems
                        .iter_mut()
                        .map(|system| system.ui(cx, world, resources))
                        .collect_vec(),
                )
            })
            .and_then(|widgets| Some(widgets))
        {
            if !widgets.is_empty() {
                return Box::new(geng::ui::stack(widgets));
            }
        }
        Box::new(ui::Void)
    }

    fn pre_update(&mut self, world: &mut legion::World, resources: &mut Resources) {
        self.systems
            .get_mut(&resources.current_state)
            .and_then(|systems| {
                Some(
                    systems
                        .iter_mut()
                        .for_each(|system| system.pre_update(world, resources)),
                )
            });
    }

    fn post_update(&mut self, world: &mut legion::World, resources: &mut Resources) {
        self.systems
            .get_mut(&resources.current_state)
            .and_then(|systems| {
                Some(
                    systems
                        .iter_mut()
                        .for_each(|system| system.post_update(world, resources)),
                )
            });
    }
}

impl GameStateSystem {
    pub fn new() -> Self {
        Self { systems: default() }
    }

    pub fn add_systems(&mut self, state: GameState, value: Vec<Box<dyn System>>) {
        let mut systems = self.systems.remove(&state).unwrap_or_default();
        systems.extend(value.into_iter());
        self.systems.insert(state, systems);
    }

    fn transition(&mut self, world: &mut legion::World, resources: &mut Resources) {
        if resources.current_state == resources.transition_state {
            return;
        }
        // transition from
        match resources.current_state {
            GameState::MainMenu => {}
            GameState::Shop => {
                Event::ShopEnd.send(world, resources);
                resources.tape_player.clear();
                ShopSystem::clear_case(world, resources);
                ShopSystem::reset_g(resources);
            }
            GameState::Battle => {
                resources.tape_player.clear();
                Event::BattleEnd.send(world, resources);
                Event::ShopStart.send(world, resources);
            }
            GameState::Gallery => {
                resources.tape_player.clear();
                resources.action_queue.clear();
                resources.status_pool.status_changes.clear();
                resources.camera.camera.fov = resources.options.fov;
                WorldSystem::set_var(world, VarName::FieldPosition, Var::Vec2(vec2(0.0, 0.0)));
            }
            GameState::GameOver => {
                resources.camera.camera.fov = resources.options.fov;
                resources.camera.focus = Focus::Battle;
            }
            GameState::CustomGame => {}
        }

        //transition to
        match resources.transition_state {
            GameState::MainMenu => {
                Game::restart(world, resources);
            }
            GameState::Battle => {
                resources.camera.focus = Focus::Battle;
                let mut tape = Some(Tape::default());
                BattleSystem::run_battle(world, resources, &mut tape);
                resources.tape_player.clear();
                resources.tape_player.tape = tape.unwrap();
            }
            GameState::Shop => {
                if resources.current_state == GameState::MainMenu {
                    ShopSystem::init_game(world, resources);
                    SlotSystem::create_entries(world, resources);
                }
                ShopSystem::init_floor(world, resources, true);
                resources.camera.focus = Focus::Shop;
            }
            GameState::Gallery => {
                resources.camera.focus = Focus::Battle;
                WorldSystem::set_var(world, VarName::FieldPosition, Var::Vec2(vec2(0.0, 20.0)));
            }
            GameState::GameOver => {
                resources.camera.focus = Focus::Shop;
                GameOverSystem::init(world, resources);
            }
            GameState::CustomGame => {
                resources.camera.focus = Focus::Battle;
                let light = resources
                    .options
                    .custom_game
                    .light
                    .clone()
                    .expect("Light team not set for custom game in options.json")
                    .into();
                let dark = resources
                    .options
                    .custom_game
                    .dark
                    .clone()
                    .expect("Light team not set for custom game in options.json")
                    .into();
                BattleSystem::clear_world(world, resources);
                BattleSystem::init_battle(&light, &dark, world, resources);
                let mut tape = Some(Tape::default());
                BattleSystem::run_battle(world, resources, &mut tape);
                let tape = tape.unwrap();
                dbg!(tape.length());
                resources.tape_player.tape = tape;
            }
        }
        resources.current_state = resources.transition_state.clone();
    }
}
