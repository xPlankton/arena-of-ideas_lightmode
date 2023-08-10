use geng::prelude::file::load_json;
use regex::Regex;
use std::{collections::VecDeque, rc::Rc};

use super::*;

mod ability;
mod ability_name;
mod ability_pool;
mod animated_shader_uniforms;
mod battle_data;
mod buff;
mod buff_pool;
mod camera;
mod condition;
mod context;
mod curse;
mod definitions;
mod effect;
mod enemy_pool;
mod event;
mod expression;
mod faction;
mod fonts;
mod gallery_data;
mod hero_pool;
mod house;
mod house_pool;
mod image;
mod image_textures;
mod input_data;
mod ladder;
mod options;
mod packed_team;
mod packed_unit;
mod rarity;
mod sacrifice_data;
mod save_data;
mod shader_programs;
mod shader_uniforms;
mod shop_data;
mod sounds;
mod status;
mod status_library;
mod tape;
mod tape_player;
mod timed_effect;
mod trigger;
mod widget;

pub use ability::*;
pub use ability_name::*;
pub use ability_pool::*;
pub use animated_shader_uniforms::*;
pub use battle_data::*;
pub use buff::*;
pub use buff_pool::*;
pub use camera::*;
pub use condition::*;
pub use context::*;
pub use curse::*;
pub use definitions::*;
pub use effect::*;
pub use enemy_pool::*;
pub use event::*;
pub use expression::*;
pub use faction::*;
pub use fonts::*;
pub use gallery_data::*;
pub use hero_pool::*;
pub use house::*;
pub use house_pool::*;
pub use image::*;
pub use image_textures::*;
pub use input_data::*;
pub use ladder::*;
pub use options::*;
pub use packed_team::*;
pub use packed_unit::*;
pub use rarity::*;
pub use sacrifice_data::*;
pub use save_data::*;
pub use shader_programs::*;
pub use shader_uniforms::*;
pub use shop_data::*;
pub use sounds::*;
pub use status::*;
pub use status_library::*;
pub use tape::*;
pub use tape_player::*;
pub use timed_effect::*;
pub use trigger::*;
pub use widget::*;

use std::cell::RefCell;
thread_local!(pub static GLOBAL_TIME: RefCell<Time> = RefCell::new(0.0));
pub struct Resources {
    pub logger: Logger,
    pub options: Options,
    pub reload_triggered: bool,

    pub shader_programs: ShaderPrograms,
    pub image_textures: ImageTextures,
    pub sounds: Sounds,

    pub global_time: Time,
    pub delta_time: Time,
    pub action_queue: VecDeque<Action>,
    pub tape_player: TapePlayer,
    pub frame_shaders: Vec<ShaderChain>,
    pub prepared_shaders: Vec<ShaderChain>,

    pub shop_data: ShopData,
    pub battle_data: BattleData,
    pub sacrifice_data: SacrificeData,
    pub input_data: InputData,
    pub panels_data: PanelsData,
    pub gallery_data: GalleryData,

    pub hero_pool: HeroPool,
    pub enemy_pool: EnemyPool,
    pub house_pool: HousePool,
    pub ability_pool: AbilityPool,
    pub buff_pool: BuffPool,
    pub curse_pool: CursePool,
    pub status_library: StatusLibrary,

    pub ladder: Ladder,
    pub definitions: Definitions,

    pub current_state: GameState,
    pub transition_state: GameState,

    pub camera: Camera,
    pub fonts: Fonts,
    pub geng: Option<Geng>,

    pub definitions_regex: Regex,
}

//todo: async loading
impl Resources {
    pub fn new(options: Options) -> Self {
        Self {
            geng: default(),
            camera: Camera::new(&options),
            fonts: default(),
            logger: default(),
            shader_programs: default(),
            image_textures: default(),
            global_time: default(),
            delta_time: default(),
            action_queue: default(),
            tape_player: default(),
            shop_data: default(),
            battle_data: default(),
            frame_shaders: default(),
            input_data: default(),
            house_pool: default(),
            definitions: default(),
            ladder: default(),
            reload_triggered: default(),
            hero_pool: default(),
            enemy_pool: default(),
            transition_state: GameState::Intro,
            current_state: GameState::Intro,
            options,
            ability_pool: default(),
            prepared_shaders: default(),
            definitions_regex: Regex::new(r"\b[A-Z][a-zA-Z]*\b").unwrap(),
            status_library: default(),
            buff_pool: default(),
            curse_pool: default(),
            sacrifice_data: default(),
            panels_data: default(),
            gallery_data: default(),
            sounds: default(),
        }
    }

    pub fn load(&mut self, watcher: &mut FileWatcherSystem) {
        watcher.watch_file(&static_path().join("options.json"), Box::new(Options::load));
        HousePool::load(self, &static_path(), watcher);
        HeroPool::load(self, &static_path().join("units/_list.json"), watcher);
        EnemyPool::load(self, &static_path().join("enemy_pool/_list.json"), watcher);
        Ladder::load(self, &static_path().join("ladder.json"), watcher);
        BuffPool::load(self, &static_path().join("buffs.json"), watcher);
        CursePool::load(self, &static_path().join("curses.json"), watcher);

        self.logger.load(&self.options);
    }

    pub fn load_geng(&mut self, watcher: &mut FileWatcherSystem, geng: &Geng) {
        dbg!("load geng");
        self.geng = Some(geng.clone());
        ShaderPrograms::shader_library_loader(
            self,
            &static_path().join("shaders/library/_list.json"),
            watcher,
        );
        self.fonts = Fonts::new(geng);
        ImageTextures::load(self, &static_path().join("images/_list.json"), watcher);
        Sounds::load(self, &static_path().join("sounds/_list.json"), watcher);
    }
}
