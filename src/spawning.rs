use std::{f32::consts::PI, time::Duration};

use bevy::{
    audio::{Volume, VolumeLevel, PlaybackMode},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_mod_picking::prelude::*;

use crate::{
    consume_counter::ConsumeCount,
    layers,
    stack::{SpawnEvent, StackPenalty},
    stress::{EmitStress, StressPopupText},
    Sfx,
};

pub struct SpawningPlugin;
impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TodayTimer {
            timer: Timer::from_seconds(10., TimerMode::Repeating),
            clicked_today: false,
        });
    }
}

#[derive(Resource)]
pub struct TodayTimer {
    timer: Timer,
    pub clicked_today: bool,
}

#[derive(Component)]
pub struct CircleButton;

#[derive(Component)]
pub struct BuyClockHand {
    bad_material: Handle<ColorMaterial>,
    happy_material: Handle<ColorMaterial>,
}

#[derive(Component)]
pub struct BuyClockKnob;

pub fn spawn_button(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            CircleButton,
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(34.)),
                    color: Color::WHITE.with_a(0.),
                    ..default()
                },
                transform: Transform::from_xyz(290., 149., layers::UI + 10.),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Click>>::commands_mut(|_, commands| {
                commands.add(SpawnEvent);
            }),
        ))
        .with_children(|children| {
            let bad_material = materials.add(ColorMaterial::from(Color::CRIMSON));
            let happy_material = materials.add(ColorMaterial::from(Color::WHITE));
            children
                .spawn((
                    BuyClockHand {
                        bad_material,
                        happy_material: happy_material.clone(),
                    },
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(19.0).into()).into(),
                        material: happy_material,
                        transform: Transform::from_translation(Vec3::new(0., 0., -2.)),
                        ..default()
                    },
                ))
                .with_children(|children| {
                    children.spawn((
                        BuyClockKnob,
                        MaterialMesh2dBundle {
                            mesh: meshes.add(shape::Circle::new(2.).into()).into(),
                            material: materials.add(ColorMaterial::from(Color::rgb(0.7, 0.7, 0.7))),
                            transform: Transform::from_translation(Vec3::new(0., 16., 1.)),
                            ..default()
                        },
                    ));
                });

            children.spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(26.)),
                    ..default()
                },
                texture: asset_server.load("buy_button.png"),
                transform: Transform::from_xyz(0., 0., -1.),
                ..default()
            });

            children.spawn(SpriteBundle {
                texture: asset_server.load("buy_carrot.png"),
                transform: Transform::from_xyz(0., 21., 0.),
                ..default()
            });
        });
}

pub fn check_timer(
    mut commands: Commands,
    mut today: ResMut<TodayTimer>,
    time: Res<Time>,
    stack_penalty: Res<StackPenalty>,
    sfx: Res<Sfx>,
    button: Query<&GlobalTransform, With<CircleButton>>,
    consumed_counter: Res<ConsumeCount>,
) {
    if today.timer.tick(time.delta()).finished() {
        let click_penalty = if today.clicked_today {
            today.clicked_today = false;
            2.
        } else {
            commands.spawn(AudioBundle {
                source: sfx.no_click.clone(),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    volume: Volume::Relative(VolumeLevel::new(0.5)),
                    ..default()
                },
            });
            5.
        };
        let stress_value = click_penalty + stack_penalty.0;
        commands.add(EmitStress(stress_value));
        commands.add(StressPopupText {
            spawn_origin: button.single().translation() - 35. * Vec3::X + 100. * Vec3::Z,
            stress_value,
        });

        // adjust timer time.
        let timer_secs = 10. - (consumed_counter.total / 10).min(5) as f32;
        today
            .timer
            .set_duration(Duration::from_secs_f32(timer_secs));
    }
}

pub fn draw_button(
    today: Res<TodayTimer>,
    mut q: Query<(&mut Transform, &BuyClockHand, &mut Handle<ColorMaterial>)>,
) {
    let fraction_left = today.timer.elapsed_secs() / today.timer.duration().as_secs_f32();

    for (mut t, materials, mut handle) in &mut q {
        t.rotation = Quat::from_rotation_z(-2. * PI * fraction_left);

        *handle = if today.clicked_today {
            materials.happy_material.clone()
        } else {
            materials.bad_material.clone()
        };
    }
}
