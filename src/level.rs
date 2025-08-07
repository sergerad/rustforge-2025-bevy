use avian2d::prelude::*;
use bevy::prelude::*;

use crate::GameState;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), setup);
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("ground"),
        Transform::from_translation(Vec3::new(0., -200., 0.)),
        RigidBody::Static,
        Collider::rectangle(600., 100.),
    ));

    let boxes = vec![
        Vec2::new(0., -130.),
        Vec2::new(0., -90.),
        Vec2::new(0., -50.),
        Vec2::new(0., -10.),
        Vec2::new(0., 30.),
        //
        Vec2::new(100., -130.),
        Vec2::new(100., -90.),
        Vec2::new(100., -50.),
        Vec2::new(100., -10.),
        Vec2::new(100., 30.),
    ];

    for b in boxes {
        commands.spawn((
            Name::new("box"),
            Transform::from_translation(Vec3::new(b.x, b.y, 0.)),
            RigidBody::Dynamic,
            Collider::rectangle(40., 40.),
        ));
    }

    commands.spawn((
        Name::new("box"),
        Transform::from_translation(Vec3::new(50., 50., 0.)),
        RigidBody::Dynamic,
        Collider::triangle(Vec2::new(-80., 0.), Vec2::new(0., 60.), Vec2::new(80., 0.)),
    ));
}
