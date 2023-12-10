use bevy::{ecs::system::Command, prelude::*};
use bevy_vector_shapes::prelude::*;

use crate::game_state::GameState;

#[derive(Component, Default)]
pub struct StressMeter {
    /// 0-100
    pub value: f32,
}

impl StressMeter {
    const DIM: Vec2 = Vec2::new(16., 130.);
    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn((
                StressMeter { value: 10. },
                ShapeBundle {
                    spatial_bundle: SpatialBundle {
                        transform: Transform::from_xyz(-300., 40., 2.),
                        ..default()
                    },
                    ..ShapeBundle::rect(
                        &ShapeConfig {
                            thickness: 1.5,
                            hollow: true,
                            corner_radii: Vec4::splat(3.),
                            ..ShapeConfig::default_2d()
                        },
                        StressMeter::DIM,
                    )
                },
            ))
            .with_children(|children| {
                children.spawn(SpriteBundle {
                    transform: Transform::from_xyz(0., Self::DIM.y / 2. + 16., 0.1),
                    texture: asset_server.load("debuff_icon.png"),
                    ..default()
                });
            });
    }

    pub fn animate_meter(
        mut painter: ShapePainter,
        stress: Query<(&StressMeter, &GlobalTransform)>,
    ) {
        let Ok((meter, transform)) = stress.get_single() else {
            return;
        };
        let height = StressMeter::DIM.y * meter.value / 100.;
        let mut translation = transform.translation();
        let bottom = transform.translation().y - StressMeter::DIM.y / 2.;
        translation.y = bottom + height / 2.;
        translation.z -= 1.;
        painter.color = Color::WHITE;
        painter.corner_radii = Vec4::splat(3.);
        painter.translate(translation);
        painter.rect(Vec2::new(StressMeter::DIM.x, height));
    }
}

pub struct EmitStress(pub f32);
impl Command for EmitStress {
    fn apply(self, world: &mut World) {
        let mut query = world.query::<&mut StressMeter>();
        let Ok(mut stress) = query.get_single_mut(world) else {
            return;
        };
        if self.0 < 0. && stress.value <= 0. {
            return;
        }
        stress.value += self.0;
    }
}

pub fn fail_state(stress: Query<&StressMeter>, mut state: ResMut<NextState<GameState>>) {
    let Ok(meter) = stress.get_single() else {
        return;
    };
    if meter.value > 100. {
        state.set(GameState::Failed);
    }
}

#[derive(Component)]
pub struct StressText {
    timer: Timer,
}

pub struct StressPopupText {
    pub spawn_origin: Vec3,
    pub stress_value: f32,
}
impl Command for StressPopupText {
    fn apply(self, world: &mut World) {
        let font = world
            .resource::<AssetServer>()
            .load("chevyray_bird_seed.ttf");
        world.spawn((
            StressText {
                timer: Timer::from_seconds(0.5, TimerMode::Once),
            },
            Text2dBundle {
                text: Text::from_section(
                    format!("{:+.0}", self.stress_value.round()),
                    TextStyle {
                        font,
                        font_size: 16.,
                        color: if self.stress_value < 0. {
                            Color::DARK_GREEN
                        } else {
                            Color::CRIMSON
                        },
                    },
                )
                .with_alignment(TextAlignment::Right),
                transform: Transform::from_translation(self.spawn_origin),
                ..default()
            },
        ));
    }
}

impl StressText {
    pub fn animate_text(
        mut commands: Commands,
        mut texts: Query<(Entity, &mut StressText)>,
        time: Res<Time>,
    ) {
        for (e, mut text) in &mut texts {
            if text.timer.tick(time.delta()).finished() {
                commands.entity(e).despawn();
            }
        }
    }
}
