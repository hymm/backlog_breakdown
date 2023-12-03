use bevy::prelude::*;

use crate::item::ItemType;

#[derive(Component, Default)]
pub struct Stack {
    item_type: ItemType,
    items: Vec<Entity>,
}

impl Stack {
    pub fn spawn(commands: &mut Commands, transform: Transform) {
        commands.spawn((
            SpatialBundle {
                transform,
                ..default()
            },
            Stack::default(),
        ));
    }
}

/// Put component on an item to label that it's on a stack
#[derive(Component)]
pub struct InStack;


