pub mod arena;
pub mod unit;
pub mod monster;
pub mod combat;
pub mod equipment;
pub mod shop;
pub mod data_loader;
pub mod party;
pub mod wave_gen;

use bevy::prelude::*;
use arena::ArenaPlugin;
use unit::UnitPlugin;
use monster::MonsterPlugin;
use combat::CombatPlugin;
use shop::ShopPlugin;
use data_loader::DataLoaderPlugin;
use party::{PlayerParty, SelectedWave};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Preparation,   // buy units from shop, review party, click Continue
    WaveSelection, // choose one of three enemy waves
    Battle,        // autobattle
}

#[derive(Resource, Default)]
pub struct Gold(pub u32);

#[derive(Resource, Default)]
pub struct WaveNumber(pub u32);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .insert_resource(Gold(10))
            .insert_resource(WaveNumber(1))
            .init_resource::<PlayerParty>()
            .init_resource::<SelectedWave>()
            .add_plugins((DataLoaderPlugin, ArenaPlugin, UnitPlugin, MonsterPlugin, CombatPlugin, ShopPlugin));
    }
}
