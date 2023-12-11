use bevy::prelude::*;

use crate::game_state::GameState;

pub struct StartScreenPlugin;
impl Plugin for StartScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::StartScreen), spawn_startup_screen)
            .add_systems(Update, (button_system, input_start))
            .add_systems(OnExit(GameState::StartScreen), despawn_menu);
    }
}

#[derive(Component)]
pub struct MenuMarker;

fn spawn_startup_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("BacklogBreakdown_Start.png"),
            ..default()
        })
        .with_children(|children| {
            children.spawn((
                MenuMarker,
                Text2dBundle {
                    text: Text::from_section(
                        "Click to Start",
                        TextStyle {
                            font: asset_server.load("chevyray_bird_seed.ttf"),
                            font_size: 16.,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ),
                    transform: Transform::from_xyz(0., -144., 1.),
                    ..default()
                },
            ));
        });

    commands
        .spawn((
            MenuMarker,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                MenuMarker,
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::DARK_GRAY.with_a(0.).into(),
                    ..default()
                },
            ));
        });
}

fn despawn_menu(mut commands: Commands, q: Query<Entity, With<MenuMarker>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}

fn button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<NextState<GameState>>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                state.set(GameState::Playing);
                // *color = Color::GRAY.with_a(0.).into();
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn input_start(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
    button_inputs: Res<Input<GamepadButton>>,
    gamepads: Res<Gamepads>,
) {
    if keyboard_input.pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::Return) {
        state.set(GameState::Playing);
    }

    for gamepad in gamepads.iter() {
        if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::Start))
            || button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::South))
        {
            state.set(GameState::Playing);
        }
    }
}
