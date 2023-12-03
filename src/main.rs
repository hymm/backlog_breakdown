// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod item;
mod queue;

use crate::item::Item;
use crate::queue::{in_queue_transforms, Queue};
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DefaultPickingPlugins))
        .add_systems(Startup, setup)
        .add_systems(Update, in_queue_transforms)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    Queue::spawn(&mut commands);

    Item::spawn(&mut commands, &asset_server);
    Item::spawn(&mut commands, &asset_server);
    Item::spawn(&mut commands, &asset_server);
    Item::spawn(&mut commands, &asset_server);
    Item::spawn(&mut commands, &asset_server);
    Item::spawn(&mut commands, &asset_server);
}
