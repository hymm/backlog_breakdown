use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::stack::{InStack, StackOffset};

#[derive(Bundle)]
pub struct ItemBundle {
    sprite_bundle: SpriteBundle,
    item_type: ItemType,
    in_stack: InStack,
    stack_offset: StackOffset,
    pickable_bundle: PickableBundle,
    on_drag_start: On<Pointer<DragStart>>,
    on_drag_end: On<Pointer<DragEnd>>,
    on_drag: On<Pointer<Drag>>,
}

impl ItemBundle {
    pub fn new(item_type: ItemType, texture: Handle<Image>, offset: f32) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
                    custom_size: Some(item_type.dimensions().yz()),
                    ..default()
                },
                // texture,
                transform: Transform::from_xyz(0., 0., 1.),
                ..default()
            },
            item_type,
            in_stack: InStack,
            stack_offset: StackOffset(offset),
            pickable_bundle: PickableBundle::default(),
            on_drag_start: On::<Pointer<DragStart>>::target_insert(Pickable::IGNORE), // Disable picking
            on_drag_end: On::<Pointer<DragEnd>>::target_insert(Pickable::default()), // Re-enable picking
            on_drag: On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
                transform.translation.x += drag.delta.x; // Make the square follow the mouse
                transform.translation.y -= drag.delta.y;
            }),
        }
    }
}

#[derive(Component, Default, Clone, Copy)]
pub enum ItemType {
    #[default]
    Book,
    Movie,
    Game,
    Comic,
}

impl ItemType {
    pub fn dimensions(&self) -> Vec3 {
        match self {
            ItemType::Book => Vec3::new(80., 100., 20.),
            ItemType::Movie => Vec3::new(60., 100., 15.),
            ItemType::Game => Vec3::new(50., 100., 10.),
            ItemType::Comic => Vec3::new(40., 100., 5.),
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ItemType::Book => "Books",
            ItemType::Movie => "Movies",
            ItemType::Game => "Games",
            ItemType::Comic => "Comics",
        }
    }
}

#[derive(Resource)]
pub struct ItemHandles {
    pub handle: Handle<Image>,
}
