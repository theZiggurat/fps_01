use bevy_rapier3d::{
    physics::{ColliderHandleComponent, RapierPhysicsPlugin}, 
    rapier::geometry::ColliderSet,
    rapier::dynamics::RigidBodySet
};
use bevy::prelude::*;


pub struct PhysicsSystemPlugin;

impl Plugin for PhysicsSystemPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_plugin(RapierPhysicsPlugin)
        .add_system(sync_physics.system());
    }
}


pub fn sync_physics(
    mut query: Query<(&ColliderHandleComponent, &mut Transform)>,
    bodies: Res<RigidBodySet>,
    colliders: Res<ColliderSet>,
    
) {
    for (handle, mut transform) in query.iter_mut() {

        if let Some(collider) = colliders.get(handle.handle()) {
            if let Some(body) = bodies.get(collider.parent()) {
                
                if body.is_dynamic() {
                    
                    let trans = body.position().translation;
                    transform.translation = Vec3::new(trans.x, trans.y, trans.z);
                    let rot = body.position().rotation;
                    transform.rotation = Quat::from_xyzw(rot.i, rot.j, rot.k, rot.w);
                }
            }
        }
        
    }
}