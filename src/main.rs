mod components;
mod module_bindings;
mod plugins;
pub mod prelude;
mod resources;
mod utils;

use bevy::log::LogPlugin;
use clap::{command, Parser, ValueEnum};
pub use prelude::*;

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    mode: RunMode,
    #[arg(short, long)]
    path: Option<String>,
}

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum RunMode {
    #[default]
    Regular,
    Custom,
    Test,
}

fn main() {
    let mut app = App::new();
    let args = Args::try_parse().unwrap_or_default();
    let target = match args.mode {
        RunMode::Regular | RunMode::Custom => GameState::CustomBattle,
        RunMode::Test => GameState::TestScenariosRun,
    };
    GameState::set_target(target);
    let default_plugins = DefaultPlugins.set(LogPlugin {
        level: bevy::log::Level::DEBUG,
        filter: "info,debug,wgpu_core=warn,wgpu_hal=warn,naga=warn".into(),
        ..default()
    });
    app.init_state::<GameState>()
        .add_plugins(default_plugins)
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Loaded)
                .load_collection::<GameAssetsHandles>()
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "ron/_dynamic.assets.ron",
                ),
        )
        .add_loading_state(
            LoadingState::new(GameState::TestScenariosLoad)
                .continue_to_state(GameState::TestScenariosRun)
                .load_collection::<TestScenarios>(),
        )
        .add_plugins(RonAssetPlugin::<GlobalSettingsAsset>::new(&[
            "global_settings.ron",
        ]))
        .add_plugins(RonAssetPlugin::<BattleData>::new(&["battle.ron"]))
        .add_plugins(RonAssetPlugin::<PackedUnit>::new(&["unit.ron"]))
        .add_plugins(RonAssetPlugin::<House>::new(&["house.ron"]))
        .add_plugins(RonAssetPlugin::<TestScenario>::new(&["scenario.ron"]))
        .add_plugins((
            LoadingPlugin,
            LoginPlugin,
            ActionPlugin,
            BattlePlugin,
            TeamPlugin,
            GameStateGraphPlugin,
            TestScenariosPlugin,
        ))
        .run();
}
