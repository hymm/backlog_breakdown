use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_rand::{prelude::ChaCha8Rng, resource::GlobalEntropy};
use rand_core::RngCore;

use crate::layers;

#[derive(Component)]
pub struct DialogBox {
    pub timer: Timer,
}

#[derive(Component)]
pub struct DialogText;

#[derive(Resource)]
pub struct ShownDialog(pub Option<&'static str>);

impl ShownDialog {
    const DIALOGS: [&'static str; 5] = [
        "Humble Bundle again...",
        "Got a gift card!",
        "Couldn't resist",
        "It wasn't on sale, but...",
        "I wanted to revist these.",
    ];

    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.insert_resource(ShownDialog(None));

        commands
            .spawn((
                DialogBox {
                    timer: Timer::from_seconds(3.5, TimerMode::Once),
                },
                SpriteBundle {
                    texture: asset_server.load("dialogbox.png"),
                    transform: Transform::from_xyz(0., 0., layers::UI + 50.),
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
        mut dialog_box: Query<(&mut Visibility, &mut DialogBox)>,
        mut dialog_text: Query<&mut Text, With<DialogText>>,
        mut shown_dialog: ResMut<ShownDialog>,
        time: Res<Time>,
    ) {
        let (mut dialog_visible, mut dialog_box) = dialog_box.single_mut();
        if let Some(ref dialog) = shown_dialog.0 {
            *dialog_visible = Visibility::Visible;
            dialog_text.single_mut().sections[0].value = dialog.to_string();
            if dialog_box.timer.tick(time.delta()).finished() {
                shown_dialog.0 = None;
            }
        } else {
            *dialog_visible = Visibility::Hidden;
        }
    }

    pub fn new_random(rng: &mut GlobalEntropy<ChaCha8Rng>) -> Self {
        let i = ((rng.next_u32() as f32 / u32::MAX as f32) * Self::DIALOGS.len() as f32 - 0.5)
            .round() as usize;
        Self(Some(Self::DIALOGS[i]))
    }
}
