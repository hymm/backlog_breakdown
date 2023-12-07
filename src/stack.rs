use bevy::{
    ecs::system::{Command, EntityCommand, SystemState},
    prelude::*,
    sprite::Anchor,
};
use bevy_rand::prelude::*;
use rand_core::RngCore;

use crate::{
    item::{ItemBundle, ItemDragging, ItemHandles, ItemType, ItemHandleIndex},
    queue::{ActiveItem, InQueue},
};

#[derive(Resource)]
struct Stacks {
    pub book_id: Entity,
    pub movie_id: Entity,
    pub game_id: Entity,
    pub comic_id: Entity,
}

#[derive(Component, Default)]
pub struct Stack {
    item_type: ItemType,
    items: Vec<Entity>,
}

impl Stack {
    fn new(item_type: ItemType) -> Self {
        Self {
            item_type,
            items: Vec::new(),
        }
    }

    pub fn spawn(commands: &mut Commands, transform: Transform, item_type: ItemType) -> Entity {
        commands
            .spawn((
                // TODO: figure out why I had to spawn a sprite to keep the text from getting cut off
                SpatialBundle {
                    transform,
                    ..default()
                },
                Stack::new(item_type),
            ))
            .with_children(|children| {
                children.spawn(Text2dBundle {
                    text: Text::from_section(item_type.label(), default()),
                    transform: Transform::from_xyz(0., -10., 0.1),
                    ..default()
                });
            })
            .id()
    }

    pub fn spawn_stacks(commands: &mut Commands) {
        let stack_y = -54.;
        let book_id = Stack::spawn(
            commands,
            Transform::from_xyz(-187., stack_y, 0.),
            ItemType::Book,
        );

        let movie_id = Stack::spawn(
            commands,
            Transform::from_xyz(-65., stack_y, 0.),
            ItemType::Movie,
        );

        let game_id = Stack::spawn(
            commands,
            Transform::from_xyz(65., stack_y, 0.),
            ItemType::Game,
        );

        let comic_id = Stack::spawn(
            commands,
            Transform::from_xyz(187., stack_y, 0.),
            ItemType::Comic,
        );

        commands.insert_resource(Stacks {
            book_id,
            movie_id,
            game_id,
            comic_id,
        });
    }
}

/// Put component on an item to label that it's on a stack
#[derive(Component)]
pub struct InStack;

/// Offset when item is in the stack
#[derive(Component)]
pub struct StackOffset(pub f32);

pub struct SpawnOn(pub ItemType);
impl Command for SpawnOn {
    fn apply(self, world: &mut World) {
        let id = match self.0 {
            ItemType::Book => world.resource::<Stacks>().book_id,
            ItemType::Movie => world.resource::<Stacks>().movie_id,
            ItemType::Game => world.resource::<Stacks>().game_id,
            ItemType::Comic => world.resource::<Stacks>().comic_id,
        };
        let mut system_state = SystemState::<(
            Commands,
            Res<ItemHandles>,
            Query<&mut Stack>,
            ResMut<GlobalEntropy<ChaCha8Rng>>,
        )>::new(world);
        let (mut commands, handles, mut query, mut rng) = system_state.get_mut(world);
        let Ok(mut stack) = query.get_mut(id) else {
            return;
        };

        let offset = ((rng.next_u32() as f32 / u32::MAX as f32) - 0.5) * 7.;
        let item_index = ((rng.next_u32() as f64 / u32::MAX as f64) * 5. - 0.5).round() as usize;
        let new_item = commands
            .spawn(ItemBundle::new(
                stack.item_type,
                stack.item_type.get_stack_handle(&handles, item_index),
                offset,
                item_index,
            ))
            .id();
        stack.items.push(new_item);
        system_state.apply(world);
    }
}

pub struct SpawnRandom;
impl Command for SpawnRandom {
    fn apply(self, world: &mut World) {
        let r = world.resource_mut::<GlobalEntropy<ChaCha8Rng>>().next_u32();
        let category = ((r as f32 / u32::MAX as f32) * 4.).trunc() as u32;
        let item_type = match category {
            0 => ItemType::Book,
            1 => ItemType::Comic,
            2 => ItemType::Game,
            3 | 4 => ItemType::Movie,
            _ => unreachable!()
        };
        SpawnOn::apply(SpawnOn(item_type), world);
    }
}

pub struct PushStack;
impl EntityCommand for PushStack {
    fn apply(self, id: Entity, world: &mut World) {
        let e = world.entity(id);
        let t = *e.get::<ItemType>().unwrap();
        let index = *e.get::<ItemHandleIndex>().unwrap();
        let handles = world.resource::<ItemHandles>();
        let new_handle = t.get_stack_handle(handles, index.0);
        let mut e = world.entity_mut(id);
        e.insert(InStack);
        *e.get_mut::<Handle<Image>>().unwrap() = new_handle;
        e.get_mut::<Sprite>().unwrap().anchor = Anchor::BottomCenter;

        let mut query = world.query::<&mut Stack>();
        let Some(mut stack) = query.find_stack(world, t) else {
            return;
        };
        stack.items.push(id);
    }
}

pub fn stack_items(
    stacks: Query<(&Stack, &GlobalTransform), Changed<Stack>>,
    mut items: Query<(&mut Transform, &StackOffset), With<InStack>>,
) {
    for (stack, transform) in &stacks {
        let offset = stack.item_type.stack_dimensions().y;
        for (i, entity) in stack.items.iter().enumerate() {
            let Ok((mut t, x_offset)) = items.get_mut(*entity) else {
                continue;
            };
            t.translation =
                transform.translation() + Vec2::new(x_offset.0, i as f32 * offset).extend(0.);
        }
    }
}

pub struct RemoveFromStack;
impl EntityCommand for RemoveFromStack {
    fn apply(self, id: Entity, world: &mut World) {
        let e = world.entity(id);
        if !e.contains::<InStack>() {
            return;
        }
        let t = *e.get::<ItemType>().unwrap();
        let mut query = world.query::<&mut Stack>();
        let Some(mut stack) = query.find_stack(world, t) else {
            return;
        };

        let i = stack.items.iter().position(|e| *e == id).unwrap();
        stack.items.remove(i);

        let handles = world.resource::<ItemHandles>();
        let item_index = world.entity(id).get::<ItemHandleIndex>().unwrap();
        let new_handle = t.get_queue_handle(handles, item_index.0);
        let mut e = world.entity_mut(id);
        e.remove::<InStack>();
        *e.get_mut::<Handle<Image>>().unwrap() = new_handle;
        e.get_mut::<Sprite>().unwrap().anchor = Anchor::Center;
    }
}

trait FindStack {
    fn find_stack<'a>(
        &'a mut self,
        world: &'a mut World,
        item_type: ItemType,
    ) -> Option<Mut<'_, Stack>>;
}

impl FindStack for QueryState<&mut Stack> {
    fn find_stack<'a>(
        &'a mut self,
        world: &'a mut World,
        item_type: ItemType,
    ) -> Option<Mut<'_, Stack>> {
        self.iter_mut(world)
            .find(|stack| stack.item_type == item_type)
    }
}

/// if an item is not in queue or stack, put it back in the stack
pub fn restack(
    mut commands: Commands,
    q: Query<
        Entity,
        (
            With<ItemType>,
            Without<InStack>,
            Without<InQueue>,
            Without<ItemDragging>,
            Without<ActiveItem>,
        ),
    >,
) {
    for e in &q {
        commands.entity(e).add(PushStack);
    }
}
