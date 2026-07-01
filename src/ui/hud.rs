use bevy::prelude::*;
use crate::game::{AppState, Gold, WaveNumber};

#[derive(Component)]
struct HudRoot;

#[derive(Component)]
struct GoldText;

#[derive(Component)]
struct WaveText;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Battle), spawn_hud)
            .add_systems(Update, update_hud.run_if(in_state(AppState::Battle)))
            .add_systems(OnExit(AppState::Battle), despawn_hud);
    }
}

fn spawn_hud(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                ..default()
            },
            HudRoot,
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("Wave: 1"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::WHITE),
                WaveText,
            ));
            p.spawn((
                Text::new("Gold: 100"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::srgb(1.0, 0.85, 0.2)),
                GoldText,
            ));
        });
}

fn update_hud(
    gold: Res<Gold>,
    wave: Res<WaveNumber>,
    mut gold_q: Query<&mut Text, (With<GoldText>, Without<WaveText>)>,
    mut wave_q: Query<&mut Text, (With<WaveText>, Without<GoldText>)>,
) {
    for mut t in &mut gold_q {
        *t = Text::new(format!("Gold: {}", gold.0));
    }
    for mut t in &mut wave_q {
        *t = Text::new(format!("Wave: {}", wave.0));
    }
}

fn despawn_hud(mut commands: Commands, query: Query<Entity, With<HudRoot>>) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}
