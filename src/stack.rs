use bevy::{
    ecs::system::{EntityCommand, SystemState},
    prelude::*,
};
use bevy_rand::prelude::*;
use rand_core::RngCore;

use crate::{
    item::{ItemBundle, ItemHandles, ItemType, ItemDragging},
    queue::InQueue,
};

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
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED, //Color::rgba_u8(0, 0, 0, 0),
                        custom_size: Some(Vec2::new(5., 5.)),
                        ..default()
                    },
                    transform,
                    ..default()
                },
                Stack::new(item_type),
            ))
            .with_children(|children| {
                children.spawn(Text2dBundle {
                    text: Text::from_section(item_type.label(), default()),
                    transform: Transform::from_xyz(0., -20., 0.),
                    ..default()
                });
            })
            .id()
    }
}

/// Put component on an item to label that it's on a stack
#[derive(Component)]
pub struct InStack;

/// Offset when item is in the stack
#[derive(Component)]
pub struct StackOffset(pub f32);

pub struct SpawnOnStack;
impl EntityCommand for SpawnOnStack {
    fn apply(self, id: Entity, world: &mut World) {
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
        let new_item = commands
            .spawn(ItemBundle::new(
                stack.item_type,
                handles.handle.stack_handle.clone(),
                offset,
            ))
            .id();
        stack.items.push(new_item);
        system_state.apply(world);
    }
}

pub struct PushStack;
impl EntityCommand for PushStack {
    fn apply(self, id: Entity, world: &mut World) {
        let handles = world.resource::<ItemHandles>();
        let new_handle = handles.handle.stack_handle.clone();
        let mut e = world.entity_mut(id);
        e.insert(InStack);
        *e.get_mut::<Handle<Image>>().unwrap() = new_handle;

        let t = *e.get::<ItemType>().unwrap();
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
        let new_handle = handles.handle.queue_handle.clone();
        let mut e = world.entity_mut(id);
        e.remove::<InStack>();
        let mut handle = e.get_mut::<Handle<Image>>().unwrap();
        *handle = new_handle;
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
        ),
    >,
) {
    for e in &q {
        commands.entity(e).add(PushStack);
    }
}
