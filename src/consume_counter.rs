use bevy::{prelude::*, utils::HashMap};

use crate::layers;

#[derive(Resource, Default)]
pub struct ConsumeCount {
    pub total: u32,
    pub books: ConsumeTypeCount,
    pub comics: ConsumeTypeCount,
    pub movies: ConsumeTypeCount,
    pub games: ConsumeTypeCount,
}

#[derive(Default)]
pub struct ConsumeTypeCount {
    pub total: u32,
    pub items: HashMap<usize, u32>,
}

impl ConsumeTypeCount {
    pub fn favorite(&self) -> Option<usize> {
        let favorite = self
            .items
            .iter()
            .fold((None, 0), |(max_key, max_value), (key, value)| {
                if max_key.is_some() {
                    if *value > max_value {
                        (Some(*key), *value)
                    } else {
                        (max_key, max_value)
                    }
                } else {
                    (Some(*key), *value)
                }
            });

        favorite.0
    }
}

#[derive(Component)]
pub struct CounterMarker;

#[derive(Component)]
pub struct CounterText;

impl CounterMarker {
    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn((
                CounterMarker,
                SpriteBundle {
                    texture: asset_server.load("score_box.png"),
                    transform: Transform::from_xyz(-280., 160., layers::UI),
                    ..default()
                },
            ))
            .with_children(|children| {
                children.spawn((
                    CounterText,
                    Text2dBundle {
                        text: Text::from_section(
                            "000000",
                            TextStyle {
                                font: asset_server.load("chevyray_bird_seed.ttf"),
                                font_size: 13.,
                                color: Color::WHITE,
                            },
                        ),
                        transform: Transform::from_xyz(0., -2., 1.),
                        ..default()
                    },
                ));
            });
    }

    pub fn despawn(mut commands: Commands, q: Query<Entity, With<CounterMarker>>) {
        for e in &q {
            commands.entity(e).despawn_recursive();
        }
    }

    pub fn update_counter(mut q: Query<&mut Text, With<CounterText>>, consumed: Res<ConsumeCount>) {
        let Ok(mut text) = q.get_single_mut() else {
            return;
        };

        text.sections[0].value = format!("{:06}", consumed.total);
    }
}
