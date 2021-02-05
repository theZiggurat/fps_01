use std::borrow::BorrowMut;

use bevy::{input::mouse::{MouseButtonInput, MouseMotion, mouse_button_input_system}, math::vec3, prelude::*, render::camera::Camera, window::CursorMoved};

use bevy_rapier3d::{na::Vector3, physics::{ColliderHandleComponent, EntityMaps, EventQueue, RapierPhysicsPlugin, RigidBodyHandleComponent}, rapier::geometry::{ColliderHandle, ColliderSet}};
use bevy_rapier3d::rapier::dynamics::{RigidBody, RigidBodyBuilder, RigidBodyHandle, RigidBodySet};
use bevy_rapier3d::rapier::geometry::ColliderBuilder;

pub struct FPSCameraPlugin;

pub struct FPSCamera {
    velocity: Vec3,
    yaw: f32,
    pitch: f32,
    enable_mouse: bool,
    enable_keyboard: bool,
    target_velocity: f32,
}

impl Default for FPSCamera {
    fn default() -> FPSCamera {
        FPSCamera {
            velocity: Vec3::zero(),
            yaw: 0.0,
            pitch: 0.0,
            enable_mouse: true,
            enable_keyboard: true,
            target_velocity: 4.0
        }
    }
}


impl Plugin for FPSCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_resource(MouseState::default())
        .add_resource(FPSCamera::default() )
        .add_system(keyboard_input_system.system())
        .add_system(mouse_input_system.system())
        .add_system(toggle_cursor.system())
        .add_system(mouse_click_system.system());
    }


}

pub fn keyboard_input_system(
    mut query: Query<(&mut FPSCamera, &mut Transform)>, 
    input: Res<Input<KeyCode>>, 
    time: Res<Time>) {

    let (mut camera, mut transform) = query.iter_mut().nth(0).unwrap();
    
    if !camera.enable_keyboard{
        return;
    }

    let (axis_f, axis_s) = (
        movement_axis(&input, KeyCode::W, KeyCode::S),
        movement_axis(&input, KeyCode::A, KeyCode::D)
    );

    let rotation = transform.rotation;
    let accel: Vec3 = (strafe_vector(&rotation) * axis_s) + (forward_vector(&rotation) * axis_f);
    let accel: Vec3 = if accel.length() != 0.0 {
        accel.normalize() * 0.8
    } else {
        Vec3::zero()
    };

    let friction: Vec3 = if camera.velocity.length() != 0.0 {
        camera.velocity.normalize() * -1.0 * 0.75
    } else {
        Vec3::zero()
    };

    camera.velocity += accel * time.delta_seconds();

    let delta_friction = friction * time.delta_seconds();
    camera.velocity = if (camera.velocity + delta_friction).signum() != camera.velocity.signum() {
        Vec3::zero()
    } else {
        camera.velocity + delta_friction
    };

    if camera.velocity.length() > camera.target_velocity {
        camera.velocity = camera.velocity.normalize() * camera.target_velocity;
    }

    transform.translation += camera.velocity;
}

#[derive(Default)]
pub struct MouseState {
    cursor_moved_event_reader: EventReader<MouseMotion>,
    mouse_button_event_reader: EventReader<MouseButtonInput>
}

pub fn mouse_input_system(
    time: Res<Time>,
    mut state: ResMut<MouseState>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mut query: Query<(&mut FPSCamera, &mut Transform)>
) {

    let (mut camera, mut transform) = query.iter_mut().nth(0).unwrap();
    if !camera.enable_mouse {
        return;
    }

    let mut delta: Vec2 = Vec2::zero();

    for event in 
        state.cursor_moved_event_reader.iter(&mouse_motion_events) {
            delta += event.delta;
    }

    if delta.is_nan() {
        return;
    }

    

    camera.yaw -= delta.x * 20.0 * time.delta_seconds();
    camera.pitch += delta.y * 20.0 * time.delta_seconds();

    camera.pitch.clamp(-89.99, 89.99);

    let yaw_radians = camera.yaw.to_radians();
    let pitch_radians = camera.pitch.to_radians();

    transform.rotation = Quat::from_axis_angle(Vec3::unit_y(), yaw_radians)
        * Quat::from_axis_angle(-Vec3::unit_x(), pitch_radians);


}

pub fn mouse_click_system(
    commands: &mut Commands,
    mouse_click_events: Res<Events<MouseButtonInput>>,
    mut state: ResMut<MouseState>,
    query: Query<(&FPSCamera, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let (_, transform) = query.iter().nth(0).unwrap();

    for event in state.mouse_button_event_reader.iter(&mouse_click_events) {
        if event.state.is_pressed() && event.button == MouseButton::Left {
            let forward: Vec3 = forward_vector(&transform.rotation) * 50.0;
            let pos = transform.translation;
        
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube{size: 1.0})),
                transform: Transform::from_translation(pos),
                ..Default::default()
            })
            .with(RigidBodyBuilder::new_dynamic().translation(pos.x, pos.y, pos.z).linvel(forward.x, forward.y, forward.z).angvel(Vector3::new(3.0, 2.0, 1.0)))
            .with(ColliderBuilder::cuboid(1.0, 1.0, 1.0));
        }
    }

    
}

pub fn movement_axis(
    input: &Res<Input<KeyCode>>,
    plus: KeyCode,
    minus: KeyCode
) -> f32 {

    let mut axis = 0.0;
    if input.pressed(plus) {
        axis += 1.0;
    }
    if input.pressed(minus) {
        axis -= 1.0;
    }
    axis
}

pub fn forward_vector(rotation: &Quat) -> Vec3 {
    rotation.mul_vec3(-Vec3::unit_z()).normalize()
}

pub fn strafe_vector(rotation: &Quat) -> Vec3 {
    forward_vector(rotation).cross(Vec3::new(0.0,-1.0,0.0))

}

fn toggle_cursor(
    input: Res<Input<KeyCode>>, 
    mut windows: ResMut<Windows>, 
    mut query: Query<&mut FPSCamera>
) {
    let window = windows.get_primary_mut().unwrap();
    if input.just_pressed(KeyCode::Escape) {

        let mut camera = query.iter_mut().nth(0).unwrap();
        camera.enable_mouse = !camera.enable_mouse;
        camera.enable_keyboard = !camera.enable_keyboard;

        window.set_cursor_lock_mode(!window.cursor_locked());
        window.set_cursor_visibility(!window.cursor_visible());

    }
}

