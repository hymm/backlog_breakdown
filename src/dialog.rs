use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_vector_shapes::prelude::*;

#[derive(Component)]
pub struct DialogBox;

#[derive(Component)]
pub struct DialogText;

#[derive(Resource)]
pub struct ShownDialog(pub Option<&'static str>);

impl ShownDialog {
    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.insert_resource(ShownDialog(None));

        commands
            .spawn((
                DialogBox,
                SpriteBundle {
                    sprite: Sprite {
                      custom_size: Some(Vec2::new(290., 22.)),
                      color: Color::DARK_GRAY,
                      ..default()
                    },
                    transform: Transform::from_xyz(0., 0., 50.),
                    ..default()
                },
                PickableBundle::default(),
                On::<Pointer<Click>>::commands_mut(|_evt, commands| {
                    commands.insert_resource(ShownDialog(None));
                }),
            ))
            .with_children(|children| {
                children.spawn((
                    DialogText,
                    Text2dBundle {
                        text: Text::from_section(
                            "Placeholder!",
                            TextStyle {
                                font: asset_server.load("chevyray_bird_seed.ttf"),
                                font_size: 16.,
                                color: Color::WHITE,
                            },
                        )
                        .with_alignment(TextAlignment::Right),
                        transform: Transform::from_xyz(0., -2., 1.),
                        ..default()
                    },
                ));
            });
    }

    pub fn despawn(mut commands: Commands, dialog_box: Query<Entity, With<DialogBox>>) {
        commands.remove_resource::<ShownDialog>();
        for e in &dialog_box {
            commands.entity(e).despawn_recursive();
        }
    }

    pub fn handle_visibility(
        mut dialog_box: Query<&mut Visibility, With<DialogBox>>,
        mut dialog_text: Query<&mut Text, With<DialogText>>,
        shown_dialog: Res<ShownDialog>,
    ) {
        if let Some(ref dialog) = shown_dialog.0 {
            *dialog_box.single_mut() = Visibility::Visible;
            dialog_text.single_mut().sections[0].value = dialog.to_string();
        } else {
            *dialog_box.single_mut() = Visibility::Hidden;
        }
    }
}
