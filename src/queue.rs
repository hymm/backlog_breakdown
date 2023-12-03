use bevy::{ecs::system::EntityCommand, prelude::*};
use bevy_mod_picking::prelude::*;

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
                transform: Transform::from_xyz(0., -300., 0.),
                ..default()
            },
            PickableBundle::default(),
            Queue::default(),
            On::<Pointer<Drop>>::commands_mut(move |event, commands| {
                commands.entity(event.dropped).add(AddToQueue);
            }),
        ));
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
    const FIRST_ITEM_OFFSET: Vec3 = Vec3::new(-125.0, 0.0, 1.0);
    let Ok((queue, queue_transform)) = queue.get_single() else {
        return;
    };

    for (index, entity) in queue.items.iter().enumerate() {
        let Ok(mut transform) = items.get_mut(*entity) else {
            continue;
        };
        transform.translation =
            queue_transform.translation() + FIRST_ITEM_OFFSET + Vec3::X * (index * 50) as f32;
    }
}
