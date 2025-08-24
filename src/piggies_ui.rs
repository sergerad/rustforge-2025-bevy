use bevy::prelude::*;

use crate::{GameState, piggies::PiggyResources};

#[derive(Component, Debug, Clone, Copy)]
struct PiggyUI;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), setup);
    app.add_systems(Update, update.run_if(in_state(GameState::Playing)));
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(0.),
            ..default()
        },
        StateScoped(GameState::Playing),
        PiggyUI,
        Text::new("Pigs: 0/0".to_string()),
        TextFont {
            font_size: 20.0,
            ..Default::default()
        },
        TextLayout::new_with_justify(JustifyText::Right).with_no_wrap(),
    ));
}

fn update(
    query: Single<Entity, (With<Text>, With<PiggyUI>)>,
    piggies: Res<PiggyResources>,
    mut text: TextUiWriter,
) {
    if piggies.is_changed() {
        let e = *query;
        *text.text(e, 0) = format!("Pigs: {}/{}", piggies.pigs, piggies.pigs_spawned);
    }
}
