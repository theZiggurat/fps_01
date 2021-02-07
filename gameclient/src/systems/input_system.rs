use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion}, 
    prelude::*
};

use bevy_rapier3d::{
    na::Vector3, 
    physics::{
        EventQueue, 
        RigidBodyHandleComponent
    }, 
    rapier::{
        geometry:: ColliderBuilder,
        dynamics::{
            RigidBodyBuilder, 
            RigidBodySet
        }
    }
};
use rapier3d::{geometry::Ball, math::Vector};
use bevy_mod_picking::{PickSource, PickingPlugin};
use na::UnitQuaternion;



pub struct FPSCameraPlugin;

pub struct FPSCamera {
    yaw: f32,
    pitch: f32,
    velocity: Vec3,
    enable_mouse: bool,
    enable_keyboard: bool,
    target_speed: f32,
    acceleration: f32,
    jump_power: f32,
}


impl Default for FPSCamera {
    fn default() -> FPSCamera {
        FPSCamera {
            yaw: 0.0,
            pitch: 0.0,
            velocity: Default::default(),
            enable_mouse: true,
            enable_keyboard: true,
            target_speed: 6.0,
            acceleration: 4.0,
            jump_power: 0.5,
        }
    }
}


impl Plugin for FPSCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_resource(MouseState::default())
        .add_startup_system(init_camera.system())
        .add_system(player_dynamics.system())
        .add_system(toggle_cursor.system())
        .add_system(mouse_click_system.system());
    }


}

pub fn init_camera(
    commands: &mut Commands,
) {

    let player_spawn = Vec3::new(0.0, 2.5, 10.0);

    let collider = ColliderBuilder::capsule_y(3.0, 1.5).density(20f32).sensor(false);
    let rigid = RigidBodyBuilder::new_dynamic()
        .translation(player_spawn.x, player_spawn.y, player_spawn.z);

    let mut cam = Camera3dBundle {
        transform: Transform::from_matrix(Mat4::from_translation(player_spawn))
            .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
    };
    cam.perspective_projection.fov = -105.0;



    commands
        .spawn(cam)
        .with(FPSCamera::default())
        .with(rigid)
        .with(collider)
        .with(PickSource::default());

}


#[derive(Default)]
pub struct MouseState {
    cursor_moved_event_reader:  EventReader<MouseMotion>,
    mouse_button_event_reader:  EventReader<MouseButtonInput>
}

pub fn player_dynamics(
    mut query:                  Query<(&mut FPSCamera, &Transform, &RigidBodyHandleComponent)>,
    mut body_set:                ResMut<RigidBodySet>,
    mut state:                  ResMut<MouseState>,
    events:                     Res<EventQueue>,
    mouse_motion_events:        Res<Events<MouseMotion>>,
    input:                      Res<Input<KeyCode>>,
    time:                       Res<Time>,
) {

    use na::{Isometry3};

    let (mut camera, transform, r_handle) = query.iter_mut().nth(0).unwrap();
    let body = body_set.get_mut(r_handle.handle()).unwrap();  
    let dt = time.delta_seconds();

    // set rotation of rigid body locked and only influenced by player input
    let rotation = pitch_yaw(&mut camera, &mut state, &mouse_motion_events, dt)
        .map(|(pitch, yaw)| UnitQuaternion::from_euler_angles( -yaw, pitch, 0.0));

    body.set_position(
        Isometry3 {
                translation: body.position().translation,
                rotation: rotation.unwrap_or(body.position().rotation)
    }, true);

    // set velocity of rigid body
    let current_velocity = body.linvel();
    let current_speed = current_velocity.xyz().magnitude();

    let accel_from_player = &accel_from_player(&mut camera, &transform, &input).unwrap_or(Vector::new(0.0,0.0,0.0));

    // combine contributions
    let combined = accel_from_player + current_velocity;
    let new_speed = combined.xyz().magnitude();
    
    let mut combined = if new_speed > camera.target_speed {
        combined.normalize() * current_speed
    } else {
        combined
    };
    combined.y = current_velocity.y;


    // jump
    let combined = if input.pressed(KeyCode::Space) {
        //  todo: ground check
        combined + Vector::new(0.0, 1.0, 0.0) * camera.jump_power
    } else {
        combined
    };

    body.set_linvel(combined, false);
    camera.velocity = Vec3::new(combined.x, combined.y, combined.z);

    body.set_angvel(Vector3::new(0.0,0.0,0.0), false);

}



pub fn accel_from_player(
    camera: &mut FPSCamera,
    transform: &Transform,
    input: &Res<Input<KeyCode>>,
) -> Option<Vector<f32>> {

    if !camera.enable_keyboard{
        return None;
    }

    let (axis_f, axis_s) = (
        movement_axis(&input, KeyCode::W, KeyCode::S),
        movement_axis(&input, KeyCode::A, KeyCode::D)
    );

    let rotation = transform.rotation;
    let accel: Vec3 = (strafe_vector(&rotation) * axis_s) + (forward_vector(&rotation) * axis_f);
    let accel: Vec3 = if accel.length() != 0.0 {
        Vec3::new(accel.x, 0.0, accel.z).normalize() * camera.acceleration
    } else {
        Vec3::zero()
    };

    Some(Vector3::new(accel.x, accel.y, accel.z))
}



pub fn pitch_yaw(
    camera: &mut FPSCamera, 
    state: &mut MouseState, 
    mouse_motion_events: &Events<MouseMotion>,
    dt: f32,
) -> Option<(f32, f32)> {

    if !camera.enable_mouse {
        return None;
    }

    let mut delta: Vec2 = Vec2::zero();

    for event in 
        state.cursor_moved_event_reader.iter(&mouse_motion_events) {
            delta += event.delta;
    }

    if delta.is_nan() {
        return None;
    }

    camera.yaw -= delta.x * 20.0 * dt;
    camera.pitch += delta.y * 20.0 * dt;
    camera.pitch = camera.pitch.clamp(-89.99, 89.99);

    Some((camera.yaw.to_radians(), camera.pitch.to_radians()))
}



pub fn mouse_click_system(
    commands: &mut Commands,
    mouse_click_events: Res<Events<MouseButtonInput>>,
    mut state: ResMut<MouseState>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&FPSCamera, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let (camera, transform) = query.iter().nth(0).unwrap();

    if !camera.enable_mouse {
        return;
    }

    let material_handle = materials.add(StandardMaterial {
        albedo: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });

    for event in state.mouse_button_event_reader.iter(&mouse_click_events) {
        if event.state.is_pressed() && event.button == MouseButton::Left {

            let forward: Vec3 = forward_vector(&transform.rotation) * 15.0;
            let pos = transform.translation + forward.normalize();

            use rand::Rng;
            let mut rng = rand::thread_rng();

            if event.button == MouseButton::Left {
                commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube {size: 2.0})),
                    material: material_handle.clone(),
                    transform: Transform::from_translation(pos),
                    ..Default::default()
                })
                .with(RigidBodyBuilder::new_dynamic()
                    .translation(pos.x, pos.y , pos.z )
                    .linvel(forward.x  + camera.velocity.x + camera.velocity.y, forward.y, forward.z + camera.velocity.z)
                    .angvel(Vector3::new(rng.gen() , rng.gen() , 1.0)))
                .with(ColliderBuilder::cuboid(1.0 ,1.0, 1.0)
                    .density(rng.gen::<f32>() * 8.0)
                    .sensor(false));
            } else if event.button == MouseButton::Right {
                commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {radius: 1.0, ..Default::default()})),
                    material: material_handle.clone(),
                    transform: Transform::from_translation(pos),
                    ..Default::default()
                })
                .with(RigidBodyBuilder::new_dynamic()
                    .translation(pos.x, pos.y , pos.z )
                    .linvel(forward.x  + camera.velocity.x + camera.velocity.y, forward.y, forward.z + camera.velocity.z)
                    .angvel(Vector3::new(rng.gen() , rng.gen() , 1.0)))
                .with(ColliderBuilder::ball(1.0)
                    .density(rng.gen::<f32>() * 8.0)
                    .sensor(false));
            }
        
            
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

