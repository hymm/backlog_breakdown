use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

pub struct Item;
impl Item {
    pub fn spawn(commands: &mut Commands, asset_server: &AssetServer) {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(50., 50.)),
                    ..default()
                },
                texture: asset_server.load("icon.png"),
                transform: Transform::from_xyz(0., 0., 1.),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<DragStart>>::target_insert(Pickable::IGNORE), // Disable picking
            On::<Pointer<DragEnd>>::target_insert(Pickable::default()), // Re-enable picking
            On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
                transform.translation.x += drag.delta.x; // Make the square follow the mouse
                transform.translation.y -= drag.delta.y;
            }),
        ));
    }
}
