pub mod menu;
pub mod hud;
pub mod shop_ui;
pub mod unit_selection;
pub mod preparation;
pub mod wave_selection;

use bevy::prelude::*;
use menu::MainMenuPlugin;
use hud::HudPlugin;
use unit_selection::UnitSelectionPlugin;
use preparation::PreparationPlugin;
use wave_selection::WaveSelectionPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MainMenuPlugin,
            HudPlugin,
            UnitSelectionPlugin,
            PreparationPlugin,
            WaveSelectionPlugin,
        ));
    }
}
