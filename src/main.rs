// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod fail_screen;
mod game_state;
mod item;
mod queue;
mod spawning;
mod stack;
mod start_screen;
mod stress;

use crate::queue::{in_queue_transforms, Queue};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_mod_picking::prelude::*;
use bevy_rand::prelude::*;
use bevy_vector_shapes::Shape2dPlugin;
use fail_screen::FailScreenPlugin;
use game_state::GameState;
use item::{ItemHandles, ItemType};
use queue::{check_active, consume_active, draw_timer};
use spawning::{check_timer, draw_button, spawn_button, SpawningPlugin};
use stack::{check_stack, restack, stack_items, Stack, StackPenalty};
use start_screen::StartScreenPlugin;
use stress::{fail_state, StressMeter};

fn main() {
    App::new()
        .add_state::<GameState>()
        .insert_resource(StackPenalty(0.))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(640., 360.),
                    ..default()
                }),
                ..default()
            }),
            DefaultPickingPlugins, // .build().disable::<DebugPickingPlugin>(),
            EntropyPlugin::<ChaCha8Rng>::default(),
            Shape2dPlugin::default(),
            SpawningPlugin,
        ))
        .add_systems(Startup, spawn_camera)
        .add_plugins((StartScreenPlugin, FailScreenPlugin))
        .add_systems(
            OnEnter(GameState::Playing),
            (
                ItemHandles::load_handles,
                (setup, StressMeter::spawn, spawn_button),
            )
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
                fail_state,
                check_stack,
                (check_timer, draw_button).chain(),
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnExit(GameState::Playing), despawn_playing)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("background.png"),
            ..default()
        },
        Pickable::IGNORE,
    ));
    Queue::spawn(&mut commands);

    Stack::spawn_stacks(&mut commands, &asset_server);
}

fn despawn_playing(
    mut commands: Commands,
    q: Query<Entity, Or<(With<ItemType>, With<Sprite>, With<Stack>, With<StressMeter>)>>,
) {
    for e in &q {
        commands.entity(e).despawn_recursive();
    }
}
