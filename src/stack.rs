use bevy::{
    ecs::system::{Command, EntityCommand, SystemState},
    prelude::*,
    sprite::Anchor,
};
use bevy_mod_picking::prelude::*;
use bevy_rand::prelude::*;
use bevy_vector_shapes::prelude::*;
use rand_core::RngCore;

use crate::{
    dialog::ShownDialog,
    item::{ItemBundle, ItemDragging, ItemHandleIndex, ItemHandles, ItemType},
    queue::{ActiveItem, InQueue},
    stress::StressMeter, spawning::TodayTimer,
};

#[derive(Component, Default)]
pub struct Stack {
    item_type: ItemType,
    items: Vec<Entity>,
    current_height: f32,
}

impl Stack {
    const MAX_HEIGHT: f32 = 240.;
    fn new(item_type: ItemType) -> Self {
        Self {
            item_type,
            items: Vec::new(),
            current_height: 0.,
        }
    }

    pub fn spawn(
        commands: &mut Commands,
        transform: Transform,
        item_type: ItemType,
        asset_server: &AssetServer,
    ) -> Entity {
        commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::CYAN.with_a(0.),
                        custom_size: Some(Vec2::new(100., Self::MAX_HEIGHT)),
                        anchor: Anchor::BottomCenter,
                        ..default()
                    },
                    transform,
                    ..default()
                },
                Stack::new(item_type),
                PickableBundle {
                    pickable: Pickable {
                        should_block_lower: true,
                        should_emit_events: true,
                    },
                    ..default()
                },
                On::<Pointer<Drop>>::commands_mut(move |event, commands| {
                    let Some(ref mut entity_commands) = commands.get_entity(event.dropped) else {
                        return;
                    };
                    entity_commands.add(AddToStack(event.target));
                }),
            ))
            .with_children(|children| {
                children.spawn(ShapeBundle {
                    spatial_bundle: SpatialBundle {
                        transform: Transform::from_xyz(0., -12., 0.5),
                        ..default()
                    },
                    ..ShapeBundle::rect(
                        &ShapeConfig {
                            color: Color::rgb_u8(217, 155, 150),
                            corner_radii: Vec4::splat(8.),
                            ..ShapeConfig::default_2d()
                        },
                        Vec2::new(60., 14.),
                    )
                });

                children.spawn(Text2dBundle {
                    text: Text::from_section(
                        item_type.label(),
                        TextStyle {
                            font: asset_server.load("chevyray_bird_seed.ttf"),
                            font_size: 12.,
                            color: Color::BLACK,
                        },
                    ),
                    transform: Transform::from_xyz(0., -14., 1.0),
                    ..default()
                });
            })
            .id()
    }

    pub fn spawn_stacks(commands: &mut Commands, asset_server: &AssetServer) {
        let stack_y = -54.;
        let book_id = Stack::spawn(
            commands,
            Transform::from_xyz(-187., stack_y, 0.1),
            ItemType::Book,
            asset_server,
        );

        let movie_id = Stack::spawn(
            commands,
            Transform::from_xyz(-65., stack_y, 0.1),
            ItemType::Movie,
            asset_server,
        );

        let game_id = Stack::spawn(
            commands,
            Transform::from_xyz(65., stack_y, 0.1),
            ItemType::Game,
            asset_server,
        );

        let comic_id = Stack::spawn(
            commands,
            Transform::from_xyz(187., stack_y, 0.1),
            ItemType::Comic,
            asset_server,
        );

        // seed the stacks
        for _ in 0..3 {
            commands.add(SpawnOn {
                item_type: ItemType::Book,
                stack_entity: book_id,
            });
        }

        for _ in 0..10 {
            commands.add(SpawnOn {
                item_type: ItemType::Comic,
                stack_entity: comic_id,
            });
        }

        for _ in 0..8 {
            commands.add(SpawnOn {
                item_type: ItemType::Game,
                stack_entity: game_id,
            });
        }

        for _ in 0..3 {
            commands.add(SpawnOn {
                item_type: ItemType::Movie,
                stack_entity: movie_id,
            });
        }
    }
}

/// Put component on an item to label that it's on a stack
#[derive(Component)]
pub struct InStack(pub Entity);

/// Offset when item is in the stack
#[derive(Component)]
pub struct StackOffset(pub f32);

struct AddToStack(pub Entity);
impl EntityCommand for AddToStack {
    fn apply(self, id: Entity, world: &mut World) {
        let e = world.entity(id);
        if !e.contains::<ItemType>() || e.contains::<InQueue>() || e.contains::<ActiveItem>() {
            return;
        }

        let mut stack = world.query::<&Stack>();
        let Ok(stack) = stack.get_mut(world, self.0) else {
            dbg!("could not find stack");
            return;
        };
        let stack_entity = if stack.current_height < Stack::MAX_HEIGHT {
            self.0
        } else {
            let Some(e) = get_random_stack(world) else {
                // there are no free stacks.
                return;
            };
            e
        };

        let mut stack = world.query::<&mut Stack>();
        let Ok(mut stack) = stack.get_mut(world, stack_entity) else {
            dbg!("could not find stack");
            return;
        };
        stack.items.push(id);
        stack_item(world, id, stack_entity);
    }
}

pub struct SpawnOn {
    pub item_type: ItemType,
    pub stack_entity: Entity,
}

impl Command for SpawnOn {
    fn apply(self, world: &mut World) {
        let mut system_state = SystemState::<(
            Commands,
            Res<ItemHandles>,
            Query<&mut Stack>,
            ResMut<GlobalEntropy<ChaCha8Rng>>,
        )>::new(world);
        let (mut commands, handles, mut query, mut rng) = system_state.get_mut(world);
        let Ok(mut stack) = query.get_mut(self.stack_entity) else {
            return;
        };

        let offset = ((rng.next_u32() as f32 / u32::MAX as f32) - 0.5) * 7.;
        let item_index = ((rng.next_u32() as f64 / u32::MAX as f64) * 5. - 0.5).round() as usize;
        let new_item = commands
            .spawn(ItemBundle::new(
                self.item_type,
                self.item_type.get_stack_handle(&handles, item_index),
                offset,
                item_index,
                self.stack_entity,
            ))
            .id();
        stack.items.push(new_item);
        stack.current_height += self.item_type.stack_dimensions().y;
        system_state.apply(world);
    }
}

fn get_random_stack(world: &mut World) -> Option<Entity> {
    let mut stacks = world.query::<(Entity, &Stack)>();
    let stacks: Vec<Entity> = stacks
        .iter(world)
        .filter_map(|(e, stack)| {
            if stack.current_height > Stack::MAX_HEIGHT {
                None
            } else {
                Some(e)
            }
        })
        .collect();

    if stacks.is_empty() {
        return None;
    }

    let mut r = world.resource_mut::<GlobalEntropy<ChaCha8Rng>>();
    let stack =
        ((r.next_u32() as f32 / u32::MAX as f32) * stacks.len() as f32 - 0.5).round() as usize;

    Some(stacks[stack])
}

pub struct SpawnEvent;

impl SpawnEvent {
    fn spawn_random(world: &mut World) -> bool {
        let mut r = world.resource_mut::<GlobalEntropy<ChaCha8Rng>>();
        let category = ((r.next_u32() as f32 / u32::MAX as f32) * 4.).trunc() as u32;
        let item_type = match category {
            0 => ItemType::Book,
            1 => ItemType::Comic,
            2 => ItemType::Game,
            3 | 4 => ItemType::Movie,
            _ => unreachable!(),
        };

        let Some(stack_entity) = get_random_stack(world) else {
            return false;
        };
        SpawnOn::apply(
            SpawnOn {
                item_type,
                stack_entity,
            },
            world,
        );

        true
    }
}

impl Command for SpawnEvent {
    fn apply(self, world: &mut World) {
        let mut r = world.resource_mut::<GlobalEntropy<ChaCha8Rng>>();
        let event = ((r.next_u32() as f32 / u32::MAX as f32) * 10. - 0.5).round() as usize == 1;
        if event {
            let event_size =
                ((r.next_u32() as f32 / u32::MAX as f32) * 5. - 0.5).round() as usize + 4;
            let mut spawned_one = false;
            for _ in 0..event_size {
                let mut r = world.resource_mut::<GlobalEntropy<ChaCha8Rng>>();
                let dialog = ShownDialog::new_random(&mut r);
                world.insert_resource(dialog);
                if !Self::spawn_random(world) {
                    // all stacks are full
                    break;
                }
                spawned_one = true;
            }
            // don't decrement the stress meter if we haven't bought anything
            if !spawned_one {
                return;
            }
        } else if !Self::spawn_random(world) {
            return;
        }

        let mut stress_meter = world.query::<&mut StressMeter>();
        let mut stress_meter = stress_meter.single_mut(world);
        stress_meter.value -= 1.;

        let mut today = world.resource_mut::<TodayTimer>();
        today.clicked_today = true;
    }
}

fn stack_item(world: &mut World, id: Entity, stack: Entity) -> ItemType {
    let e = world.entity(id);
    let t = *e.get::<ItemType>().unwrap();
    let index = *e.get::<ItemHandleIndex>().unwrap();
    let handles = world.resource::<ItemHandles>();
    let new_handle = t.get_stack_handle(handles, index.0);
    let mut e = world.entity_mut(id);
    e.insert(InStack(stack));
    *e.get_mut::<Handle<Image>>().unwrap() = new_handle;
    e.get_mut::<Sprite>().unwrap().anchor = Anchor::BottomCenter;

    t
}

pub fn stack_items(
    mut stacks: Query<(&mut Stack, &Transform), Changed<Stack>>,
    mut items: Query<(&mut Transform, &StackOffset, &ItemType), (With<InStack>, Without<Stack>)>,
) {
    for (mut stack, transform) in &mut stacks {
        let mut current_height = 0.;
        for entity in stack.items.iter() {
            let Ok((mut t, x_offset, item_type)) = items.get_mut(*entity) else {
                continue;
            };
            t.translation =
                transform.translation + Vec2::new(x_offset.0, current_height).extend(1.);
            current_height += item_type.stack_dimensions().y;
        }
        stack.current_height = current_height;
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
        let InStack(stack_id) = *e.get::<InStack>().unwrap();
        let mut query = world.query::<&mut Stack>();
        let Ok(mut stack) = query.get_mut(world, stack_id) else {
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

// if an item is not in queue or stack, put it back in the stack
pub fn restack(
    mut commands: Commands,
    free_items: Query<
        Entity,
        (
            With<ItemType>,
            Without<InStack>,
            Without<InQueue>,
            Without<ItemDragging>,
            Without<ActiveItem>,
        ),
    >,
    stacks: Query<Entity, With<Stack>>,
    mut rng: ResMut<GlobalEntropy<ChaCha8Rng>>,
) {
    let stacks: Vec<Entity> = stacks.iter().collect();
    for e in &free_items {
        let rand_stack = (4. * rng.next_u32() as f32 / u32::MAX as f32 - 0.5).round() as usize;
        commands.entity(e).add(AddToStack(stacks[rand_stack]));
    }
}

#[derive(Resource)]
pub struct StackPenalty(pub f32);

pub fn check_stack(
    stacks: Query<(&Stack, &Children)>,
    items_query: Query<&ItemType, With<InStack>>,
    mut rects: Query<&mut Rectangle>,
    mut stack_penalty: ResMut<StackPenalty>,
) {
    let mut penalty = 0.;
    for (
        Stack {
            item_type, items, ..
        },
        children,
    ) in &stacks
    {
        let rect_entity = children.iter().next();
        let mut text_stack_has_penalty = false;
        for item in items.iter() {
            if items_query.get(*item).unwrap() != item_type {
                text_stack_has_penalty = true;
                penalty += 0.5;
            }
        }

        if let Some(rect_entity) = rect_entity {
            let Ok(ref mut rect) = rects.get_mut(*rect_entity) else {
                continue;
            };

            rect.color = if text_stack_has_penalty {
                Color::CRIMSON
            } else {
                Color::rgb_u8(217, 155, 150)
            };
        }
    }
    stack_penalty.0 = penalty;
}

pub fn blink(color_1: Color, color_2: Color, time: f32) {}
