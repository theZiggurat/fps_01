use bevy_rapier3d::{
    physics::{EventQueue, ColliderHandleComponent, RapierPhysicsPlugin}, 
    rapier::geometry::ColliderSet,
    rapier::dynamics::RigidBodySet
};
use bevy::prelude::*;

use super::FPSCamera;


pub struct PhysicsSystemPlugin;

impl Plugin for PhysicsSystemPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_plugin(RapierPhysicsPlugin)
        .add_system(display_events.system());
        // .add_system(sync_physics.system());
        // .add_system(sync_player.system());
    }
}

fn display_events(events: Res<EventQueue>, colliders: Res<ColliderSet>) {

    while let Ok(intersection_event) = events.intersection_events.pop() {

        println!("{:?}", intersection_event);
        let c1 = colliders.get(intersection_event.collider1).unwrap();
        let c2 = colliders.get(intersection_event.collider2).unwrap();

        // println!("{:?} {:?}", c1., c2.shape());

    }

    while let Ok(contact_event) = events.contact_events.pop() {
        println!("Received contact event: {:?}", contact_event);
    }
}


pub fn sync_physics(
    mut query: Query<(&ColliderHandleComponent, &mut Transform), Without<FPSCamera>>,
    bodies: Res<RigidBodySet>,
    colliders: Res<ColliderSet>,
    
) {
    // for (handle, mut transform) in query.iter_mut() {

    //     if let Some(collider) = colliders.get(handle.handle()) {
    //         if let Some(body) = bodies.get(collider.parent()) {
                
    //             //if body.is_dynamic() {
                    
    //                 let trans = body.position().translation;
    //                 transform.translation = Vec3::new(trans.x, trans.y, trans.z);
    //                 let rot = body.position().rotation;
    //                 transform.rotation = Quat::from_xyzw(rot.i, rot.j, rot.k, rot.w);
                    
                
    //         }
    //     }
        
    // }
}

// pub fn sync_player(
//     mut query: Query<(&ColliderHandleComponent, &FPSCamera, &mut Transform)>,
//     bodies: Res<RigidBodySet>,
//     colliders: Res<ColliderSet>,
// ) {
//     let (handle, _, mut transform) = query.iter_mut().nth(0).unwrap_or(return);
//     if let Some(collider) = colliders.get(handle.handle()) {
//         if let Some(body) = bodies.get(collider.parent()) {
            
                
//             let trans = body.position().translation;
//             transform.translation = Vec3::new(trans.x, trans.y, trans.z);
//             let rot = body.position().rotation;
//             transform.rotation = Quat::from_xyzw(rot.i, rot.j, rot.k, rot.w);
                
            
//         }
//     }
// }