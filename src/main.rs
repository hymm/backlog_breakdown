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
use item::{ItemHandles, ItemType, ItemHandle};
use stack::{stack_items, Stack, SpawnOnStack};
use bevy_rand::prelude::*;

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
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (in_queue_transforms, stack_items))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("background.png"),
        ..default()
    });
    Queue::spawn(&mut commands);

    let id = Stack::spawn(
        &mut commands,
        Transform::from_xyz(0., 0., 0.),
        ItemType::Book,
    );

    commands.insert_resource(ItemHandles { handle: ItemHandle {
        stack_handle: asset_server.load("Book1_side.png"),
        queue_handle: asset_server.load("Book1_cover.png"),
    } });

    commands.entity(id).add(SpawnOnStack);
    commands.entity(id).add(SpawnOnStack);
    commands.entity(id).add(SpawnOnStack);
    commands.entity(id).add(SpawnOnStack);
}
