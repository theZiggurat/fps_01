use bevy_rapier3d::{physics::{EventQueue, ColliderHandleComponent, InteractionPairFilters, RapierPhysicsPlugin}, rapier::dynamics::RigidBodySet, rapier::geometry::{PairFilterContext, ContactPairFilter, SolverFlags}};
use bevy::prelude::*;

use super::FPSCamera;


pub struct PhysicsSystemPlugin;

impl Plugin for PhysicsSystemPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_plugin(RapierPhysicsPlugin)
        .add_system(display_events.system())
        .add_startup_system(setup_physics.system());
        // .add_system(sync_physics.system());
        // .add_system(sync_player.system());
    }
}

struct CustomFilter;
impl ContactPairFilter for CustomFilter {
    fn filter_contact_pair(&self, context: &PairFilterContext) -> Option<SolverFlags> {
        println!("here");
        if (context.rigid_body1.is_static() && context.rigid_body2.is_kinematic()) || (context.rigid_body2.is_static() && context.rigid_body1.is_kinematic()) {
            println!("here");
        }
        if !context.rigid_body1.is_static() && !context.rigid_body2.is_static() {
            Some(SolverFlags::COMPUTE_IMPULSES)
        } else {
            None
        }
    }
}

fn setup_physics(commands: &mut Commands) {
    commands
        .insert_resource(InteractionPairFilters::new().contact_filter(CustomFilter));
}

fn display_events(events: Res<EventQueue>) {

    while let Ok(intersection_event) = events.intersection_events.pop() {
        println!("Received intersection event: {:?}", intersection_event);

    }

    while let Ok(contact_event) = events.contact_events.pop() {
        println!("Received contact event: {:?}", contact_event);
    }
}


// pub fn sync_physics(
//     mut query: Query<(&ColliderHandleComponent, &mut Transform), Without<FPSCamera>>,
//     bodies: Res<RigidBodySet>,
//     colliders: Res<ColliderSet>,
    
// ) {
//     // for (handle, mut transform) in query.iter_mut() {

//     //     if let Some(collider) = colliders.get(handle.handle()) {
//     //         if let Some(body) = bodies.get(collider.parent()) {
                
//     //             //if body.is_dynamic() {
                    
//     //                 let trans = body.position().translation;
//     //                 transform.translation = Vec3::new(trans.x, trans.y, trans.z);
//     //                 let rot = body.position().rotation;
//     //                 transform.rotation = Quat::from_xyzw(rot.i, rot.j, rot.k, rot.w);
                    
                
//     //         }
//     //     }
        
//     // }
// }

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