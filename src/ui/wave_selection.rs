use bevy::prelude::*;
use crate::game::{
    AppState, WaveNumber,
    party::SelectedWave,
    monster::MonsterKind,
    data_loader::RoundsDatabase,
    wave_gen::WaveOption,
};

#[derive(Resource, Default)]
pub struct WaveOptions(pub Vec<WaveOption>);

#[derive(Component)]
struct WaveSelectionRoot;

#[derive(Component)]
struct WaveChoiceButton(usize);

pub struct WaveSelectionPlugin;

impl Plugin for WaveSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WaveOptions>()
            .add_systems(
                OnEnter(AppState::WaveSelection),
                (prepare_options, spawn_ui).chain(),
            )
            .add_systems(
                Update,
                handle_choice.run_if(in_state(AppState::WaveSelection)),
            )
            .add_systems(OnExit(AppState::WaveSelection), despawn_ui);
    }
}

fn prepare_options(
    wave:      Res<WaveNumber>,
    rounds_db: Res<RoundsDatabase>,
    mut opts:  ResMut<WaveOptions>,
) {
    let round_num = wave.0.clamp(1, 100);
    let Some(round) = rounds_db.0.get(&round_num) else {
        warn!("No round data for wave {}", wave.0);
        return;
    };

    opts.0 = round.options.iter().map(|opt| WaveOption {
        label: opt.label.clone(),
        monsters: opt.monsters.iter()
            .filter_map(|m| MonsterKind::from_name(&m.name).map(|k| (k, m.count)))
            .filter(|(_, n)| *n > 0)
            .collect(),
    }).collect();
}

fn spawn_ui(mut commands: Commands, options: Res<WaveOptions>, wave: Res<WaveNumber>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(50.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.04, 0.04, 0.08, 0.97)),
            WaveSelectionRoot,
        ))
        .with_children(|root| {
            root.spawn((
                Text::new(format!("Round {} — Choose Your Enemy", wave.0)),
                TextFont { font_size: 40.0, ..default() },
                TextColor(Color::WHITE),
            ));

            root.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(30.0),
                    ..default()
                },
            ))
            .with_children(|row| {
                for (i, opt) in options.0.iter().enumerate() {
                    spawn_wave_card(row, i, opt);
                }
            });
        });
}

fn spawn_wave_card(parent: &mut ChildBuilder, index: usize, option: &WaveOption) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(260.0),
                min_height: Val::Px(180.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(20.0)),
                row_gap: Val::Px(12.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.18, 0.08, 0.08)),
            BorderColor(Color::srgb(0.6, 0.2, 0.2)),
            WaveChoiceButton(index),
        ))
        .with_children(|card| {
            card.spawn((
                Text::new(&option.label),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::WHITE),
            ));
            card.spawn((
                Text::new(option.describe()),
                TextFont { font_size: 14.0, ..default() },
                TextColor(Color::srgb(0.85, 0.6, 0.6)),
            ));
            card.spawn((
                Text::new("Click to select"),
                TextFont { font_size: 12.0, ..default() },
                TextColor(Color::srgb(0.45, 0.45, 0.45)),
            ));
        });
}

fn handle_choice(
    mut selected:   ResMut<SelectedWave>,
    mut next_state: ResMut<NextState<AppState>>,
    options:        Res<WaveOptions>,
    interactions:   Query<(&Interaction, &WaveChoiceButton), Changed<Interaction>>,
) {
    for (interaction, button) in &interactions {
        if *interaction == Interaction::Pressed {
            selected.monsters = options.0[button.0].monsters.clone();
            next_state.set(AppState::Battle);
        }
    }
}

fn despawn_ui(mut commands: Commands, query: Query<Entity, With<WaveSelectionRoot>>) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}
