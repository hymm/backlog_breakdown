use std::time::Duration;

use bevy::{prelude::*, sprite::Anchor};
use bevy_mod_picking::prelude::*;

use crate::{
    layers,
    stack::{InStack, RemoveFromStack, StackOffset},
};

#[derive(Bundle)]
pub struct ItemBundle {
    sprite_bundle: SpriteBundle,
    item_type: ItemType,
    item_index: ItemHandleIndex,
    in_stack: InStack,
    stack_offset: StackOffset,
    pickable_bundle: PickableBundle,
    on_drag_start: On<Pointer<DragStart>>,
    on_drag_end: On<Pointer<DragEnd>>,
    on_drag: On<Pointer<Drag>>,
}

impl ItemBundle {
    pub fn new(
        item_type: ItemType,
        texture: Handle<Image>,
        offset: f32,
        item_index: usize,
        stack_entity: Entity,
    ) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    anchor: Anchor::BottomCenter,
                    // custom_size: Some(item_type.stack_dimensions()),
                    ..default()
                },
                texture,
                transform: Transform::from_xyz(0., 0., layers::ITEMS),
                ..default()
            },
            item_type,
            item_index: ItemHandleIndex(item_index),
            in_stack: InStack(stack_entity),
            stack_offset: StackOffset(offset),
            pickable_bundle: PickableBundle::default(),
            on_drag_start: On::<Pointer<DragStart>>::commands_mut(|evt, commands| {
                commands
                    .entity(evt.target)
                    .insert((Pickable::IGNORE, ItemDragging))
                    .add(RemoveFromStack);
            }),
            on_drag_end: On::<Pointer<DragEnd>>::commands_mut(|evt, commands| {
                commands
                    .entity(evt.target)
                    .insert(Pickable::default())
                    .remove::<ItemDragging>();
            }),
            on_drag: On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
                transform.translation.x += drag.delta.x; // Make the square follow the mouse
                transform.translation.y -= drag.delta.y;
                transform.translation.z = layers::ITEMS + 1.;
            }),
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct ItemHandleIndex(pub usize);

#[derive(Component, Default, Clone, Copy, PartialEq, Debug)]
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
            ItemType::Comic => Vec2::new(43., 8.),
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

    pub fn consume_time(&self) -> Duration {
        match self {
            ItemType::Book => Duration::from_secs_f32(5.),
            ItemType::Movie => Duration::from_secs_f32(2.),
            ItemType::Game => Duration::from_secs_f32(8.),
            ItemType::Comic => Duration::from_secs_f32(1.),
        }
    }

    pub fn get_stack_handle(&self, handles: &ItemHandles, index: usize) -> Handle<Image> {
        match self {
            ItemType::Book => handles.books[index].stack_handle.clone(),
            ItemType::Movie => handles.movies[index].stack_handle.clone(),
            ItemType::Game => handles.games[index].stack_handle.clone(),
            ItemType::Comic => handles.comics[index].stack_handle.clone(),
        }
    }

    pub fn get_queue_handle(&self, handles: &ItemHandles, index: usize) -> Handle<Image> {
        match self {
            ItemType::Book => handles.books[index].queue_handle.clone(),
            ItemType::Movie => handles.movies[index].queue_handle.clone(),
            ItemType::Game => handles.games[index].queue_handle.clone(),
            ItemType::Comic => handles.comics[index].queue_handle.clone(),
        }
    }

    pub fn get_handles_max(&self, handles: &ItemHandles) -> usize {
        match self {
            ItemType::Book => handles.books.len(),
            ItemType::Movie => handles.movies.len(),
            ItemType::Game => handles.games.len(),
            ItemType::Comic => handles.comics.len(),
        }
    }
}

#[derive(Resource)]
pub struct ItemHandles {
    pub books: Vec<ItemHandle>,
    pub movies: Vec<ItemHandle>,
    pub games: Vec<ItemHandle>,
    pub comics: Vec<ItemHandle>,
}

impl ItemHandles {
    pub fn load_handles(mut commands: Commands, asset_server: Res<AssetServer>) {
        let mut books = vec![];
        let mut movies = vec![];
        let mut comics = vec![];
        let mut games = vec![];

        for i in 1..=25 {
            books.push(ItemHandle {
                stack_handle: asset_server.load(format!("Books/Book{i}_side.png")),
                queue_handle: asset_server.load(format!("Books/Book{i}_cover.png")),
            });

            movies.push(ItemHandle {
                stack_handle: asset_server.load(format!("Movies/Movie{i}_side.png")),
                queue_handle: asset_server.load(format!("Movies/Movie{i}_cover.png")),
            });

            games.push(ItemHandle {
                stack_handle: asset_server.load(format!("Games/Game{i}_side.png")),
                queue_handle: asset_server.load(format!("Games/Game{i}_cover.png")),
            });

            comics.push(ItemHandle {
                stack_handle: asset_server.load(format!("Comics/Comic{i}_side.png")),
                queue_handle: asset_server.load(format!("Comics/Comic{i}_cover.png")),
            });
        }

        commands.insert_resource(ItemHandles {
            books,
            movies,
            comics,
            games,
        });
    }
}

pub struct ItemHandle {
    pub stack_handle: Handle<Image>,
    pub queue_handle: Handle<Image>,
}

#[derive(Component)]
pub struct ItemDragging;
