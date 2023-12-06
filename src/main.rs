// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod item;
mod queue;
mod stack;

use crate::queue::{in_queue_transforms, Queue};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_mod_picking::prelude::*;
use bevy_rand::prelude::*;
use bevy_vector_shapes::Shape2dPlugin;
use item::{ItemHandle, ItemHandles, ItemType};
use queue::{check_active, consume_active, draw_timer};
use stack::{restack, stack_items, Stack, SpawnOn};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(640., 360.),
                    ..default()
                }),
                ..default()
            }),
            DefaultPickingPlugins,
            EntropyPlugin::<ChaCha8Rng>::default(),
            Shape2dPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                in_queue_transforms,
                stack_items,
                restack,
                check_active,
                consume_active,
                draw_timer,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("background.png"),
        ..default()
    });
    Queue::spawn(&mut commands);

    commands.insert_resource(ItemHandles {
        book: ItemHandle {
            stack_handle: asset_server.load("Book1_side.png"),
            queue_handle: asset_server.load("Book1_cover.png"),
        },
        movie: ItemHandle {
            stack_handle: asset_server.load("Movie1_side.png"),
            queue_handle: asset_server.load("Movie1_cover.png"),
        },
        comic: ItemHandle {
            stack_handle: asset_server.load("Comic1_side.png"),
            queue_handle: asset_server.load("Comic1_cover.png"),
        },
        game: ItemHandle {
            stack_handle: asset_server.load("Game1_side.png"),
            queue_handle: asset_server.load("Game1_cover.png"),
        },
    });


    Stack::spawn_stacks(&mut commands);

    for _ in 0..3 {
        commands.add(SpawnOn(ItemType::Book));    
    }

    for _ in 0..10 {
        commands.add(SpawnOn(ItemType::Comic));    
    }
    
    for _ in 0..8 {
        commands.add(SpawnOn(ItemType::Game));    
    }
    
    for _ in 0..3 {
        commands.add(SpawnOn(ItemType::Movie));    
    }
}
