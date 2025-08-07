use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{GameState, loading::GameAssets};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, on_shoot.run_if(in_state(GameState::Playing)));
}

fn on_shoot(
    mut commands: Commands,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    assets: Res<GameAssets>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        commands.spawn((
            Name::new("bird"),
            Transform::from_translation(Vec3::new(-200., 20., 0.)),
            RigidBody::Dynamic,
            LinearVelocity(Vec2::new(1000., 50.)),
            Collider::circle(20.),
            children![(
                Transform::from_scale(Vec3::new(0.2, 0.2, 1.)),
                Sprite::from_image(assets.bevy.clone()),
            )],
        ));
    }
}
