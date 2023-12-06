use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_vector_shapes::prelude::*;

use crate::stack::SpawnRandom;

pub struct SpawningPlugin;
impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(5., TimerMode::Repeating)))
            .add_systems(Startup, spawn_button)
            .add_systems(Update, (check_timer, draw_button).chain());
    }
}

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

#[derive(Component)]
pub struct CircleButton;

pub fn spawn_button(mut commands: Commands) {
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
                commands.add(SpawnRandom);
            }),
        ))
        .with_children(|children| {
            children.spawn(ShapeBundle::circle(
                &ShapeConfig {
                    color: Color::BLUE,
                    thickness: 3.,
                    hollow: true,
                    ..ShapeConfig::default_2d()
                },
                17.,
            ));
        });
}

pub fn check_timer(mut timer: ResMut<SpawnTimer>, time: Res<Time>) {
    if timer.0.tick(time.delta()).finished() {
        dbg!("tick");
    }
}

pub fn draw_button(
    q: Query<&GlobalTransform, With<CircleButton>>,
    timer: Res<SpawnTimer>,
    mut painter: ShapePainter,
) {
    let Ok(transform) = q.get_single() else {
        return;
    };
    let fraction_left = timer.0.elapsed_secs() / timer.0.duration().as_secs_f32();

    painter.translate(transform.translation().xy().extend(9.));
    // painter.thickness = 0.5;
    painter.hollow = false;
    painter.color = Color::WHITE;
    painter.cap = Cap::None;
    painter.arc(15., 0., 2. * PI * fraction_left);
}
