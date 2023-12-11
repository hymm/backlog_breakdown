// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod consume_counter;
mod dialog;
mod fail_screen;
mod game_state;
mod item;
mod layers;
mod queue;
mod spawning;
mod stack;
mod start_screen;
mod stress;

use crate::queue::{in_queue_transforms, Queue};
use bevy::window::WindowResolution;
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_mod_picking::prelude::*;
use bevy_rand::prelude::*;
use consume_counter::{ConsumeCount, CounterMarker};
use dialog::ShownDialog;
use fail_screen::FailScreenPlugin;
use game_state::GameState;
use item::{ItemHandles, ItemType};
use queue::{check_active, consume_active, draw_timer};
use spawning::{check_timer, draw_button, spawn_button, SpawningPlugin};
use stack::{check_stack, restack, stack_items, Stack, StackPenalty};
use start_screen::StartScreenPlugin;
use stress::{fail_state, StressMeter, StressText};

fn main() {
    App::new()
        .add_state::<GameState>()
        .insert_resource(StackPenalty(0.))
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ConsumeCount::default())
        .insert_resource(Msaa::Off)
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(640., 360.),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            DefaultPickingPlugins
                .build()
                .disable::<DebugPickingPlugin>(),
            EntropyPlugin::<ChaCha8Rng>::default(),
            SpawningPlugin,
        ))
        .add_systems(Startup, (spawn_camera, ItemHandles::load_handles))
        .add_plugins((StartScreenPlugin, FailScreenPlugin))
        .add_systems(
            OnEnter(GameState::Playing),
            ((
                setup,
                StressMeter::spawn,
                spawn_button,
                CounterMarker::spawn,
                ShownDialog::spawn,
                BackgroundMusic::spawn,
                Queue::spawn,
            ),)
                .chain(),
        )
        .add_systems(
            Update,
            (
                in_queue_transforms,
                stack_items,
                restack,
                check_active,
                consume_active,
                draw_timer,
                StressMeter::animate_meter,
                StressMeter::animate_stress_overlays,
                fail_state,
                check_stack,
                CounterMarker::update_counter,
                ShownDialog::handle_visibility,
                StressText::animate_text,
                skip_to_end,
                (check_timer, draw_button).chain(),
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            OnExit(GameState::Playing),
            (
                despawn_playing,
                CounterMarker::despawn,
                ShownDialog::despawn,
                BackgroundMusic::despawn,
            ),
        )
        .run();
}

#[derive(Resource)]
pub struct Sfx {
    no_click: Handle<AudioSource>,
    buy: Handle<AudioSource>,
    consume: Handle<AudioSource>,
}

fn spawn_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(Sfx {
        no_click: asset_server.load("sfx/no-click.ogg"),
        buy: asset_server.load("sfx/buy.ogg"),
        consume: asset_server.load("sfx/consume.ogg"),
    });
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    commands.insert_resource(ConsumeCount::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("background.png"),
            ..default()
        },
        Pickable::IGNORE,
    ));

    Stack::spawn_stacks(&mut commands, &asset_server, &mut rng);
}

fn despawn_playing(
    mut commands: Commands,
    q: Query<
        Entity,
        Or<(
            With<ItemType>,
            With<Sprite>,
            With<Stack>,
            With<StressMeter>,
            With<Text>,
        )>,
    >,
) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}

#[derive(Component)]
struct BackgroundMusic;

impl BackgroundMusic {
    fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((
            BackgroundMusic,
            AudioBundle {
                source: asset_server.load("carefully-does-it.ogg"),
                settings: PlaybackSettings::LOOP,
            },
        ));
    }

    fn despawn(mut commands: Commands, q: Query<Entity, With<BackgroundMusic>>) {
        for e in &q {
            commands.entity(e).despawn();
        }
    }
}

fn skip_to_end(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
    button_inputs: Res<Input<GamepadButton>>,
    gamepads: Res<Gamepads>,
) {
    if keyboard_input.pressed(KeyCode::Escape) || keyboard_input.pressed(KeyCode::Return) {
        state.set(GameState::Failed);
    }

    for gamepad in gamepads.iter() {
        if button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::Start))
            || button_inputs.pressed(GamepadButton::new(gamepad, GamepadButtonType::South))
        {
            state.set(GameState::Failed);
        }
    }
}
