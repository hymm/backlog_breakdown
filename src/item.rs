use std::time::Duration;

use bevy::{prelude::*, sprite::Anchor};
use bevy_mod_picking::prelude::*;

use crate::stack::{InStack, RemoveFromStack, StackOffset};

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
                    anchor: Anchor::BottomCenter,
                    // custom_size: Some(item_type.stack_dimensions()),
                    ..default()
                },
                texture,
                transform: Transform::from_xyz(0., 0., 1.),
                ..default()
            },
            item_type,
            in_stack: InStack,
            stack_offset: StackOffset(offset),
            pickable_bundle: PickableBundle::default(),
            on_drag_start: On::<Pointer<DragStart>>::commands_mut(|evt, commands| {
                commands
                    .entity(evt.target)
                    .insert((Pickable::IGNORE, ItemDragging))
                    .add(RemoveFromStack);
            }), // Disable picking
            on_drag_end: On::<Pointer<DragEnd>>::commands_mut(|evt, commands| {
                commands
                    .entity(evt.target)
                    .insert(Pickable::default())
                    .remove::<ItemDragging>();
            }),
            on_drag: On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
                transform.translation.x += drag.delta.x; // Make the square follow the mouse
                transform.translation.y -= drag.delta.y;
            }),
        }
    }
}

#[derive(Component, Default, Clone, Copy, PartialEq)]
pub enum ItemType {
    #[default]
    Book,
    Movie,
    Game,
    Comic,
}

impl ItemType {
    pub fn stack_dimensions(&self) -> Vec2 {
        match self {
            ItemType::Book => Vec2::new(65., 17.),
            ItemType::Movie => Vec2::new(65., 10.),
            ItemType::Game => Vec2::new(65., 12.),
            ItemType::Comic => Vec2::new(65., 8.),
        }
    }

    // pub fn queue_dimensions(&self) -> Vec2 {
    //     match self {
    //         ItemType::Book => Vec2::new(45., 60.),
    //         ItemType::Movie => Vec2::new(37., 49.),
    //         ItemType::Game => Vec2::new(34., 55.),
    //         ItemType::Comic => Vec2::new(33., 43.),
    //     }
    // }

    pub fn label(&self) -> &'static str {
        match self {
            ItemType::Book => "Books",
            ItemType::Movie => "Movies",
            ItemType::Game => "Games",
            ItemType::Comic => "Comics",
        }
    }

    pub fn consume_time(&self) -> Duration {
        match self {
            ItemType::Book => Duration::from_secs_f32(5.),
            ItemType::Movie => Duration::from_secs_f32(2.),
            ItemType::Game => Duration::from_secs_f32(10.),
            ItemType::Comic => Duration::from_secs_f32(1.),
        }
    }

    pub fn get_stack_handle(&self, handles: &ItemHandles) -> Handle<Image> {
        match self {
            ItemType::Book => handles.book.stack_handle.clone(),
            ItemType::Movie => handles.movie.stack_handle.clone(),
            ItemType::Game => handles.game.stack_handle.clone(),
            ItemType::Comic => handles.comic.stack_handle.clone(),
        }
    }

    pub fn get_queue_handle(&self, handles: &ItemHandles) -> Handle<Image> {
        match self {
            ItemType::Book => handles.book.queue_handle.clone(),
            ItemType::Movie => handles.movie.queue_handle.clone(),
            ItemType::Game => handles.game.queue_handle.clone(),
            ItemType::Comic => handles.comic.queue_handle.clone(),
        }
    }
}

#[derive(Resource)]
pub struct ItemHandles {
    pub book: ItemHandle,
    pub movie: ItemHandle,
    pub game: ItemHandle,
    pub comic: ItemHandle,
}

pub struct ItemHandle {
    pub stack_handle: Handle<Image>,
    pub queue_handle: Handle<Image>,
}

#[derive(Component)]
pub struct ItemDragging;
