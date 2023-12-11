use bevy::{ecs::system::Command, prelude::*, sprite::Anchor};

use crate::game_state::GameState;

#[derive(Component, Default)]
pub struct StressMeter {
    /// 0-100
    pub value: f32,
}

#[derive(Component)]
pub struct StressMeterRect;

impl StressMeter {
    const DIM: Vec2 = Vec2::new(16., 130.);
    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn((
                StressMeter { value: 10. },
                SpriteBundle {
                    texture: asset_server.load("meter_stress.png"),
                    transform: Transform::from_xyz(-299., 41., 2.),
                    ..default()
                },
            ))
            .with_children(|children| {
                children.spawn(SpriteBundle {
                    transform: Transform::from_xyz(2., Self::DIM.y / 2. + 18., 0.1),
                    texture: asset_server.load("stress_icon.png"),
                    ..default()
                });

                children.spawn((
                    StressMeterRect,
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(14., 1.)),
                            color: Color::rgb_u8(108, 58, 70),
                            anchor: Anchor::BottomCenter,
                            ..default()
                        },
                        transform: Transform::from_xyz(0., -57., -0.1),
                        ..default()
                    },
                ));
            });
    }

    pub fn animate_meter(
        stress: Query<&StressMeter>,
        mut stress_rect: Query<&mut Sprite, With<StressMeterRect>>,
    ) {
        let height = 114. * stress.single().value / 100.;
        let Some(ref mut size) = stress_rect.single_mut().custom_size else {
            return;
        };
        size.y = height.max(1.);
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

#[derive(Component)]
pub struct StartPosition(pub Vec3);

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
                timer: Timer::from_seconds(2., TimerMode::Once),
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
                ),
                transform: Transform::from_translation(self.spawn_origin),
                ..default()
            },
            StartPosition(self.spawn_origin),
        ));
    }
}

impl StressText {
    pub fn animate_text(
        mut commands: Commands,
        mut texts: Query<(Entity, &mut StressText, &mut Transform, &mut Text)>,
        time: Res<Time>,
    ) {
        for (e, mut stress_timer, mut t, mut text) in &mut texts {
            t.translation.y += time.delta().as_secs_f32() * 10.;
            let new_a = text.sections[0].style.color.a() - 0.005 * text.sections[0].style.color.a();
            text.sections[0].style.color.set_a(new_a.max(0.0));
            if stress_timer.timer.tick(time.delta()).finished() {
                commands.entity(e).despawn();
            }
        }
    }
}
