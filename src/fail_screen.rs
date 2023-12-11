use bevy::prelude::*;

use crate::{consume_counter::ConsumeCount, game_state::GameState, item::ItemHandles};

pub struct FailScreenPlugin;
impl Plugin for FailScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Failed), spawn)
            .add_systems(OnExit(GameState::Failed), despawn);
    }
}

#[derive(Component)]
struct FailMarker;

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    counts: Res<ConsumeCount>,
    handles: Res<ItemHandles>,
) {
    commands.spawn((FailMarker, SpriteBundle {
        texture: asset_server.load("BacklogBreakdown_GameOver.png"),
        ..default()
    }));

    commands
        .spawn((
            FailMarker,
            NodeBundle {
                style: Style {
                    width: Val::Px(200.),
                    height: Val::Percent(100.),
                    margin: UiRect::px(390., 0., 40., 0.),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|children| {
            children.spawn((
                FailMarker,
                TextBundle::from_section(
                    "You've drowned in your backlog!",
                    TextStyle {
                        font: asset_server.load("chevyray_bird_seed.ttf"),
                        font_size: 16.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
            ));

            children.spawn((
                FailMarker,
                TextBundle::from_section(
                    format!(
                        "\nTotal: {}\nBooks: {}\nMovies: {}\nGames: {}\nComics: {}",
                        counts.total,
                        counts.books.total,
                        counts.movies.total,
                        counts.games.total,
                        counts.comics.total
                    ),
                    TextStyle {
                        font: asset_server.load("chevyray_bird_seed.ttf"),
                        font_size: 16.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
            ));

            children
                .spawn((
                    FailMarker,
                    NodeBundle {
                        style: Style {
                            margin: UiRect::top(Val::Px(65.)),
                            height: Val::Px(90.),
                            row_gap: Val::Px(12.),
                            column_gap: Val::Px(14.),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|children| {
                    if let Some(index) = counts.books.favorite() {
                        children.spawn((
                            FailMarker,
                            ImageBundle {
                                image: UiImage {
                                    texture: handles.books[index].queue_handle.clone(),
                                    ..default()
                                },
                                ..default()
                            },
                        ));
                    }

                    if let Some(index) = counts.movies.favorite() {
                        children.spawn((
                            FailMarker,
                            ImageBundle {
                                image: UiImage {
                                    texture: handles.movies[index].queue_handle.clone(),
                                    ..default()
                                },
                                ..default()
                            },
                        ));
                    }

                    if let Some(index) = counts.games.favorite() {
                        children.spawn((
                            FailMarker,
                            ImageBundle {
                                image: UiImage {
                                    texture: handles.games[index].queue_handle.clone(),
                                    ..default()
                                },
                                ..default()
                            },
                        ));
                    }

                    if let Some(index) = counts.comics.favorite() {
                        children.spawn((
                            FailMarker,
                            ImageBundle {
                                image: UiImage {
                                    texture: handles.comics[index].queue_handle.clone(),
                                    ..default()
                                },
                                ..default()
                            },
                        ));
                    }
                });

            children
                .spawn((
                    FailMarker,
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(40.0),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::DARK_GRAY.into(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        FailMarker,
                        TextBundle::from_section(
                            "Click to Replay",
                            TextStyle {
                                font: asset_server.load("chevyray_bird_seed.ttf"),
                                font_size: 20.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                    ));
                });
        });
}

fn despawn(mut commands: Commands, q: Query<Entity, With<FailMarker>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}
