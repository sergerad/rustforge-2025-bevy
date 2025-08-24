use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use crate::{GameState, loading::GameAssets};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::SplashScreen), setup);
}

fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    info!("setup menu");
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            StateScoped(GameState::SplashScreen),
        ))
        .with_children(|parent: &mut RelatedSpawnerCommands<ChildOf>| {
            parent.spawn((
                Node {
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![
                    bevy(&assets),
                    (
                        Node::default(),
                        Text::new("Angry Bevys".to_string()),
                        TextFont {
                            font_size: 20.0,
                            ..Default::default()
                        },
                        TextLayout::new_with_justify(JustifyText::Center).with_no_wrap(),
                    ),
                    bevy(&assets),
                ],
            ));

            parent
                .spawn((
                    Node {
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    children![play_button()],
                ))
                .observe(on_play_button);
        });
}

fn on_play_button(_: Trigger<Pointer<Click>>, mut next_game_state: ResMut<NextState<GameState>>) {
    info!("Play button pressed");
    next_game_state.set(GameState::Playing);
}

fn bevy(assets: &Res<GameAssets>) -> impl Bundle {
    (
        Node {
            width: Val::Px(40.),
            ..default()
        },
        ImageNode::new(assets.bevy.clone()),
    )
}

fn play_button() -> impl Bundle {
    (
        Node::default(),
        Button,
        Text::new("Play".to_string()),
        TextFont {
            font_size: 20.0,
            ..Default::default()
        },
        TextLayout::new_with_justify(JustifyText::Center).with_no_wrap(),
    )
}
