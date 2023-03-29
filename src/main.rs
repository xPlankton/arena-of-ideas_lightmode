use geng::prelude::*;
use geng::ui;

mod components;
pub mod game;
mod resources;
mod systems;

use anyhow::{Error, Result};
use components::*;
use game::*;
use legion::query::*;
use legion::EntityStore;
use resources::{Resources, *};
use std::path::PathBuf;
use systems::*;

type Time = f32;

fn setup_geng() -> Geng {
    geng::setup_panic_handler();
    let geng = Geng::new_with(geng::ContextOptions {
        title: "Arena of Ideas".to_owned(),
        antialias: true,
        shader_prefix: Some((
            include_str!("vertex_prefix.glsl").to_owned(),
            include_str!("fragment_prefix.glsl").to_owned(),
        )),
        target_ui_resolution: Some(vec2(1920.0, 1080.0)),
        window_size: Some(vec2(1920, 1080)),
        ..default()
    });
    geng
}

fn static_path() -> PathBuf {
    run_dir().join("static")
}
fn save_path() -> PathBuf {
    run_dir().join("save")
}
fn main() {
    let timer = Instant::now();
    logger::init();

    let options = Options::load();
    let mut world = legion::World::default();
    let mut resources = Resources::new(options);

    let mut watcher = FileWatcherSystem::new();
    resources.load(&mut watcher);
    let geng = setup_geng();
    resources.load_geng(&mut watcher, &geng);
    Game::init_world(&mut resources, &mut world);

    let mut theme = geng.ui_theme();
    theme.font = resources.fonts.get_font(1);
    theme.hover_color = Rgba::BLACK;
    geng.set_ui_theme(theme);
    if resources.options.walkthrough {
        WalkthroughSystem::run_simulation(&mut world, &mut resources);
    } else {
        let game = Game::new(world, resources, watcher);
        debug!("Game load in: {:?}", timer.elapsed());
        geng.clone().run(game);
    }
}
