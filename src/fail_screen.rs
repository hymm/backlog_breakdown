use bevy::prelude::*;

use crate::game_state::GameState;

pub struct FailScreenPlugin;
impl Plugin for FailScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Failed), spawn)
            .add_systems(OnExit(GameState::Failed), despawn);
    }
}

#[derive(Component)]
struct FailMarker;

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            FailMarker,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|children| {
            children.spawn((
                FailMarker,
                TextBundle::from_section(
                    "You Failed!",
                    TextStyle {
                        font: asset_server.load("chevyray_bird_seed.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
            ));
        });
}

fn despawn(mut commands: Commands, q: Query<Entity, With<FailMarker>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}
