use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{GameState, loading::GameAssets, piggies::PiggySpawn};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), setup);
}

fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    // ground
    commands.spawn((
        Name::new("ground"),
        StateScoped(GameState::Playing),
        Transform::from_translation(Vec3::new(0., -200., 0.)),
        RigidBody::Static,
        Collider::rectangle(1800., 100.),
    ));

    let boxes = vec![
        // house 1
        Vec2::new(0., -130.),
        Vec2::new(0., -90.),
        Vec2::new(0., -50.),
        Vec2::new(0., -10.),
        // Vec2::new(0., 30.),
        //
        Vec2::new(100., -130.),
        Vec2::new(100., -90.),
        Vec2::new(100., -50.),
        Vec2::new(100., -10.),
        // Vec2::new(100., 30.),
        // house 2
        Vec2::new(400., -130.),
        Vec2::new(400., -90.),
        Vec2::new(400., -50.),
        Vec2::new(400., -10.),
        Vec2::new(400., 30.),
        //
        Vec2::new(500., -130.),
        Vec2::new(500., -90.),
        Vec2::new(500., -50.),
        Vec2::new(500., -10.),
        Vec2::new(500., 30.),
    ];

    let piggies = vec![
        // house 1
        Vec2::new(50., -130.),
        // house 2
        Vec2::new(450., -130.),
    ];

    for b in boxes {
        commands.spawn((
            Name::new("box"),
            StateScoped(GameState::Playing),
            Transform::from_translation(Vec3::new(b.x, b.y, 0.)),
            RigidBody::Dynamic,
            Collider::rectangle(40., 40.),
            children![(
                Transform::from_scale(Vec3::new(0.3, 0.3, 1.)),
                Sprite::from_image(assets.kiste.clone()),
            )],
        ));
    }

    commands.spawn((
        Name::new("roof"),
        StateScoped(GameState::Playing),
        Transform::from_translation(Vec3::new(50., 50., 0.)),
        RigidBody::Dynamic,
        Collider::triangle(Vec2::new(-80., 0.), Vec2::new(0., 80.), Vec2::new(80., 0.)),
        children![(
            Transform::from_scale(Vec3::new(1.1, 1.1, 1.)).with_translation(Vec3::new(0., 40., 0.)),
            Sprite::from_image(assets.roof.clone()),
        )],
    ));

    commands.trigger(PiggySpawn(piggies));
}
