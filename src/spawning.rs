use std::f32::consts::PI;

use bevy::{
    audio::{Volume, VolumeLevel},
    prelude::*,
};
use bevy_mod_picking::prelude::*;
use bevy_vector_shapes::prelude::*;

use crate::{
    stack::{SpawnEvent, StackPenalty},
    stress::{EmitStress, StressPopupText},
    Sfx, dialog::{DialogBox, DialogText, ShownDialog},
};

pub struct SpawningPlugin;
impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TodayTimer {
            timer: Timer::from_seconds(5., TimerMode::Repeating),
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

pub fn spawn_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            CircleButton,
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(34.)),
                    color: Color::WHITE.with_a(0.),
                    ..default()
                },
                transform: Transform::from_xyz(290., 149., 10.),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Click>>::commands_mut(|_, commands| {
                commands.add(SpawnEvent);
            }),
        ))
        .with_children(|children| {
            children.spawn(ShapeBundle::circle(
                &ShapeConfig {
                    color: Color::DARK_GRAY,
                    thickness: 2.,
                    hollow: true,
                    ..ShapeConfig::default_2d()
                },
                20.,
            ));

            children.spawn(SpriteBundle {
                transform: Transform::from_xyz(0., 0., 1.),
                texture: asset_server.load("Sale_icon.png"),
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
) {
    if today.timer.tick(time.delta()).finished() {
        let click_penalty = if today.clicked_today {
            today.clicked_today = false;
            2.
        } else {
            commands.spawn(AudioBundle {
                source: sfx.no_click.clone(),
                settings: PlaybackSettings {
                    volume: Volume::Relative(VolumeLevel::new(0.5)),
                    ..default()
                },
            });
            5.
        };
        let stress_value = click_penalty + stack_penalty.0;
        commands.add(EmitStress(stress_value));
        commands.add(StressPopupText {
            spawn_origin: button.single().translation() - 35. * Vec3::X,
            stress_value,
        });
    }
}

pub fn draw_button(
    q: Query<&GlobalTransform, With<CircleButton>>,
    today: Res<TodayTimer>,
    mut painter: ShapePainter,
) {
    let Ok(transform) = q.get_single() else {
        return;
    };
    let fraction_left = today.timer.elapsed_secs() / today.timer.duration().as_secs_f32();

    painter.translate(transform.translation().xy().extend(9.));
    // painter.thickness = 0.5;
    painter.hollow = false;
    painter.color = if today.clicked_today {
        Color::WHITE
    } else {
        Color::CRIMSON
    };
    painter.cap = Cap::None;
    painter.arc(18., 0., 2. * PI * fraction_left);
}
