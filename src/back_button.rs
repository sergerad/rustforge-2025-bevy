/// This simply adds a UI containing a back button
/// in case the Player gets stuck and wants to abort the level
use bevy::prelude::*;

use crate::GameState;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), setup);
}

fn setup(mut commands: Commands) {
    info!("setup menu");
    commands
        .spawn((
            Node::default(),
            StateScoped(GameState::Playing),
            Button,
            Text::new("Back".to_string()),
            TextFont {
                font_size: 20.0,
                ..Default::default()
            },
            TextLayout::new_with_justify(JustifyText::Center).with_no_wrap(),
        ))
        .observe(on_back_button);
}

fn on_back_button(_: Trigger<Pointer<Click>>, mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::SplashScreen);
}
