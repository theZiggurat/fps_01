use std::borrow::BorrowMut;

use bevy::{input::mouse::{MouseButtonInput, MouseMotion, mouse_button_input_system}, math::vec3, prelude::*, render::camera::Camera, window::CursorMoved};

use bevy_rapier3d::{na::{Isometry, Translation3, Vector3}, physics::{ColliderHandleComponent, EntityMaps, EventQueue, RapierPhysicsPlugin, RigidBodyHandleComponent}, rapier::{crossbeam::thread, geometry::{ColliderHandle, ColliderSet, Shape, SharedShape}, parry::query::PointQuery}};
use bevy_rapier3d::rapier::dynamics::{RigidBody, RigidBodyBuilder, RigidBodyHandle, RigidBodySet};
use bevy_rapier3d::rapier::geometry::ColliderBuilder;
use na::{Point3, Rotation3, UnitQuaternion};


pub struct FPSCameraPlugin;

pub struct FPSCamera {
    velocity: Vec3,
    yaw: f32,
    pitch: f32,
    enable_mouse: bool,
    enable_keyboard: bool,
    target_velocity: f32,
    gravity: f32,
}


impl Default for FPSCamera {
    fn default() -> FPSCamera {
        FPSCamera {
            velocity: Vec3::zero(),
            yaw: 0.0,
            pitch: 0.0,
            enable_mouse: true,
            enable_keyboard: true,
            target_velocity: 4.0,
            gravity: -9.8
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

    let collider = ColliderBuilder::capsule_y(3.0, 1.5).density(20f32).sensor(true);
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
        .with(collider);

}


#[derive(Default)]
pub struct MouseState {
    cursor_moved_event_reader:  EventReader<MouseMotion>,
    mouse_button_event_reader:  EventReader<MouseButtonInput>
}

pub fn player_dynamics(
    mut query:                  Query<(&mut FPSCamera, &mut Transform, &RigidBodyHandleComponent, &ColliderHandleComponent)>,
    mut bodySet:                ResMut<RigidBodySet>,
    mut state:                  ResMut<MouseState>,
    events:                     Res<EventQueue>,
    colliders:                  Res<ColliderSet>,
    mouse_motion_events:        Res<Events<MouseMotion>>,
    input:                      Res<Input<KeyCode>>,
    time:                       Res<Time>,
) {

    use na::{Isometry3};

    let (mut camera, mut transform, r_handle, c_handle) = query.iter_mut().nth(0).unwrap();
    let mut body = bodySet.get_mut(r_handle.handle()).unwrap();  
    let position = body.position();
    let dt = time.delta_seconds();

    let rotation = pitch_yaw(&mut camera, &mut state, &mouse_motion_events, dt)
        .map(|(pitch, yaw)| UnitQuaternion::from_euler_angles( -yaw, pitch, 0.0));

    let (translation, velocity) = velocity(&mut camera,  &transform, &input, dt).map(|v|  {

        while let Ok(intersection_event) = events.intersection_events.pop() {
            
            let (player, other) = {
                let c1 = colliders.get(intersection_event.collider1).unwrap();
                let c2 = colliders.get(intersection_event.collider2).unwrap();

                if c_handle.handle() == intersection_event.collider1 {
                    (c1, c2)
                } else if c_handle.handle() == intersection_event.collider2 {
                    (c2, c1)
                } else {
                    continue;
                }

                
            };

            let (point, feature) = other.shape().project_point_and_get_feature(other.position(), &(player.position().translation * Point3::new(0.0,0.0,0.0)));
            let norm = other.shape().feature_normal_at_point(feature, &point.point);

            println!("{:?}", norm);




    
        }

        //Translation3::new(v.x + position.translation.x, v.y + position.translation.y + (camera.gravity * dt), v.z + position.translation.z)
        (Translation3::new(v.x + position.translation.x, v.y + position.translation.y, v.z + position.translation.z),
        Vector3::new())
        //Vector3::new(v.x , v.y + (camera.gravity * dt * 50.0), v.z )
    });

    body.set_position(
        Isometry3 {
                translation: translation.unwrap_or(body.position().translation),
                rotation: rotation.unwrap_or(body.position().rotation)
    }, true);

    body.set_linvel(Vector3::new(), true);

    // body.set_linvel(translation.unwrap_or(*body.linvel() + *body.linvel()), true)

    // body.set_position(
    //     Isometry3 {
    //             translation: body.position().translation,
    //             rotation: rotation.unwrap_or(body.position().rotation)
    //     }   
    // , true);
 
    // body.set_linvel(translation.unwrap_or(*body.linvel() + *body.linvel()), true)

}


pub fn velocity(
    camera: &mut FPSCamera,
    transform: &Transform,
    input: &Res<Input<KeyCode>>,
    dt: f32,
) -> Option<Translation3<f32>> {

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
        accel.normalize() * 0.8
    } else {
        Vec3::zero()
    };

    let friction: Vec3 = if camera.velocity.length() != 0.0 {
        camera.velocity.normalize() * -1.0 * 0.75
    } else {
        Vec3::zero()
    };

    camera.velocity += accel * dt;

    let delta_friction = friction * dt;
    camera.velocity = if (camera.velocity + delta_friction).signum() != camera.velocity.signum() {
        Vec3::zero()
    } else {
        camera.velocity + delta_friction
    };

    if input.pressed(KeyCode::Space) {
        camera.velocity += Vec3::unit_y() * 0.10;
    }

    if camera.velocity.length() > camera.target_velocity {
        camera.velocity = camera.velocity.normalize() * camera.target_velocity;
    }
    let v = camera.velocity * 30f32;

    Some(Translation3::new(v.x, v.y, v.z))
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
    camera.pitch.clamp(-89.99, 89.99);

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
    let (_, transform) = query.iter().nth(0).unwrap();

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

            //let collider = ColliderBuilder::new(SharedShape::cuboid(0.5, 0.5, 0.5))

        
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube{size: 1.0})),
                material: material_handle.clone(),
                transform: Transform::from_translation(pos),
                ..Default::default()
            })
            .with(RigidBodyBuilder::new_dynamic().translation(pos.x, pos.y, pos.z).linvel(forward.x, forward.y, forward.z).angvel(Vector3::new(rng.gen() , rng.gen() , 1.0)))
            .with(ColliderBuilder::cuboid(0.5, 0.5, 0.5).density(rng.gen::<f32>() * 8.0).sensor(false));
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

