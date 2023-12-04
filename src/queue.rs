use std::f32::consts::PI;

use bevy::{
    ecs::system::{Command, EntityCommand},
    prelude::*,
};
use bevy_mod_picking::prelude::*;
use bevy_vector_shapes::prelude::*;

use crate::item::ItemType;

#[derive(Component)]
pub struct InQueue;

#[derive(Component, Default)]
pub struct Queue {
    items: Vec<Entity>,
}

impl Queue {
    pub fn spawn(commands: &mut Commands) {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(300., 50.)),
                    ..default()
                },
                transform: Transform::from_xyz(0., -130., 0.),
                ..default()
            },
            PickableBundle::default(),
            Queue::default(),
            On::<Pointer<Drop>>::commands_mut(move |event, commands| {
                commands.entity(event.dropped).add(AddToQueue);
            }),
        ));

        ConsumeActive::spawn(commands);
    }
}

struct AddToQueue;
impl EntityCommand for AddToQueue {
    fn apply(self, id: Entity, world: &mut World) {
        let mut queue = world.query::<&mut Queue>();
        let mut queue = queue.single_mut(world);
        queue.items.push(id);

        world.entity_mut(id).insert((InQueue, Pickable::IGNORE));
    }
}

pub fn in_queue_transforms(
    mut items: Query<&mut Transform, With<InQueue>>,
    queue: Query<(&Queue, &GlobalTransform)>,
) {
    const FIRST_ITEM_OFFSET: Vec3 = Vec3::new(125.0, 0.0, 1.0);
    let Ok((queue, queue_transform)) = queue.get_single() else {
        return;
    };

    for (index, entity) in queue.items.iter().enumerate() {
        let Ok(mut transform) = items.get_mut(*entity) else {
            continue;
        };
        transform.translation =
            queue_transform.translation() + FIRST_ITEM_OFFSET - Vec3::X * (index * 50) as f32;
    }
}

/// visual for active item
#[derive(Component)]
pub struct ConsumeActive;

impl ConsumeActive {
    pub fn spawn(commands: &mut Commands) {
        commands.spawn((
            ConsumeActive,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(50., 50.)),
                    ..default()
                },
                transform: Transform::from_xyz(200., -130., 0.),
                ..default()
            },
        ));
    }
}

/// Marker component for the active item.
#[derive(Component)]
pub struct ActiveItem(pub Timer);

struct PopQueue;
impl Command for PopQueue {
    fn apply(self, world: &mut World) {
        let mut queues = world.query::<&mut Queue>();
        let Ok(mut queue) = queues.get_single_mut(world) else {
            return;
        };
        let Some(active_item) = queue.items.pop() else {
            return;
        };

        let mut active_slot = world.query_filtered::<&GlobalTransform, With<ConsumeActive>>();
        let active_slot_translation = active_slot.single(world).translation();

        let mut e = world.entity_mut(active_item);
        let item_type = *e.get::<ItemType>().unwrap();
        let mut transform = e.get_mut::<Transform>().unwrap();
        transform.translation = active_slot_translation;
        e.remove::<InQueue>().insert(ActiveItem(Timer::new(
            item_type.comsume_time(),
            TimerMode::Once,
        )));
    }
}

pub fn check_active(mut commands: Commands, active_query: Query<(), With<ActiveItem>>) {
    if active_query.is_empty() {
        commands.add(PopQueue);
    }
}

pub fn consume_active(
    mut commands: Commands,
    mut active_query: Query<(Entity, &mut ActiveItem)>,
    time: Res<Time>,
) {
    let Ok((e, mut timer)) = active_query.get_single_mut() else {
        return;
    };
    if timer.0.tick(time.delta()).just_finished() {
        commands.entity(e).despawn();
    }
}

pub fn draw_timer(
    mut painter: ShapePainter,
    active_query: Query<&mut ActiveItem>,
    active_slot: Query<&GlobalTransform, With<ConsumeActive>>,
) {
    let Ok(timer) = active_query.get_single() else {
        return;
    };
    let Ok(transform) = active_slot.get_single() else {
        return;
    };

    let fraction_left = timer.0.elapsed_secs() / timer.0.duration().as_secs_f32();

    painter.translate(transform.translation().xy().extend(3.));
    painter.thickness = 0.5;
    painter.hollow = false;
    painter.color = Color::GREEN;
    painter.cap = Cap::None;
    painter.arc(20., 0., 2. * PI * fraction_left);
}
