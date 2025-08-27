/// This plugin defines the sling shot mechanic of the birds.
/// The `Sling` resource holds the position and velocity of the sling.
/// This way we can worry about seperating working on the trajectory visualizing and the sling shot input mechanics.
use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{GameState, MainCamera, loading::GameAssets};

#[derive(Resource)]
struct Sling {
    pub pos: Vec2,
    pub is_aiming: bool,
    pub aim_start_pos: Vec2,
    pub active_touch_id: Option<u64>,
}

#[derive(Event)]
struct OnShoot {
    velocity: Vec2,
}

pub fn plugin(app: &mut App) {
    app.insert_resource(Sling {
        pos: Vec2::new(-300., -40.),
        is_aiming: false,
        aim_start_pos: Vec2::ZERO,
        active_touch_id: None,
    });

    app.add_systems(OnEnter(GameState::Playing), setup);
    app.add_systems(
        Update,
        (on_mouse_input, touch_input).run_if(in_state(GameState::Playing)),
    );

    app.add_observer(on_shoot);
}

fn setup(mut commands: Commands, sling: Res<Sling>, assets: Res<GameAssets>) {
    commands.spawn((
        Name::new("sling"),
        StateScoped(GameState::Playing),
        Transform::from_translation(sling.pos.extend(0.)),
        children![(
            Transform::from_scale(Vec3::new(0.2, 0.2, 1.)),
            Sprite::from_image(assets.bevy.clone()),
        )],
    ));
}

fn touch_input(mut commands: Commands, touches: Res<Touches>, mut sling: ResMut<Sling>) {
    if touches.any_just_pressed() && !sling.is_aiming {
        sling.is_aiming = true;
        if let Some(position) = touches.first_pressed_position() {
            sling.aim_start_pos = position;
            // Get the first pressed touch ID
            for touch in touches.iter() {
                sling.active_touch_id = Some(touch.id());
                break;
            }
        }
    }

    if let Some(touch_id) = sling.active_touch_id {
        if touches.just_released(touch_id) && sling.is_aiming {
            if let Some(touch) = touches.get_released(touch_id) {
                let touch_pos = touch.position();
                let velocity = calculate_velocity_from_positions(touch_pos, sling.pos);
                commands.trigger(OnShoot { velocity });
                sling.is_aiming = false;
                sling.active_touch_id = None;
            }
        }
    }
}

fn on_mouse_input(
    mut commands: Commands,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut sling: ResMut<Sling>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        sling.is_aiming = true;
        if let Ok(window) = windows.single() {
            if let Some(cursor_pos) = window.cursor_position() {
                if let Ok((camera, camera_transform)) = camera_q.single() {
                    if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos)
                    {
                        sling.aim_start_pos = world_pos;
                    }
                }
            }
        }
    }

    if mouse_buttons.just_released(MouseButton::Left) && sling.is_aiming {
        if let Ok(window) = windows.single() {
            if let Some(cursor_pos) = window.cursor_position() {
                if let Ok((camera, camera_transform)) = camera_q.single() {
                    if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos)
                    {
                        let velocity = calculate_velocity_from_positions(world_pos, sling.pos);
                        commands.trigger(OnShoot { velocity });
                    }
                }
            }
        }
        sling.is_aiming = false;
    }
}

fn calculate_velocity_from_positions(end_pos: Vec2, sling_pos: Vec2) -> Vec2 {
    // Calculate the pull vector from sling position to the mouse release position
    let pull_vector = end_pos - sling_pos;

    // The launch direction is opposite to the pull direction (like a real slingshot)
    let launch_direction = -pull_vector;

    // Scale velocity based on pull distance - further pull = more power
    let pull_distance = pull_vector.length();
    let velocity_multiplier = (pull_distance * 2.0).min(1200.0).max(200.0); // Cap between 200 and 1200

    // Apply velocity multiplier to launch direction
    if pull_distance > 10.0 {
        launch_direction.normalize() * velocity_multiplier
    } else {
        // Default forward velocity if there's minimal pull
        Vec2::new(800.0, 100.0)
    }
}

fn on_shoot(
    trigger: Trigger<OnShoot>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    sling: Res<Sling>,
) {
    let velocity = trigger.event().velocity;

    commands.spawn((
        Name::new("bird"),
        StateScoped(GameState::Playing),
        Transform::from_translation(sling.pos.extend(0.)),
        RigidBody::Dynamic,
        LinearVelocity(velocity),
        Collider::circle(20.),
        children![(
            Transform::from_scale(Vec3::new(0.2, 0.2, 1.)),
            Sprite::from_image(assets.bevy.clone()),
        )],
    ));
}
