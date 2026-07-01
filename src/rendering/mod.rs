pub mod placeholder;

use bevy::prelude::*;
use placeholder::PlaceholderPlugin;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlaceholderPlugin);
    }
}
