use bevy_rapier3d::{physics::{ColliderHandleComponent, EntityMaps, EventQueue, RapierPhysicsPlugin, RigidBodyHandleComponent}, rapier::geometry::{ColliderHandle, ColliderSet}};
use bevy_rapier3d::rapier::dynamics::{RigidBody, RigidBodyBuilder, RigidBodyHandle, RigidBodySet};
use bevy_rapier3d::rapier::geometry::ColliderBuilder;
use bevy_rapier3d::render::RapierRenderPlugin;
use bevy::prelude::*;

pub struct PhysicsSystem {

}

pub struct PhysicsSystemPlugin;

impl Plugin for PhysicsSystemPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_plugin(RapierPhysicsPlugin)
        .add_system(sync_physics.system())
        .add_startup_system(sync_physics_initial.system());
    }
}

pub fn sync_physics_initial() {

}

pub fn sync_physics(
    mut query: Query<(Entity, &ColliderHandleComponent, &mut Transform)>,
    bodies: Res<RigidBodySet>,
    colliders: Res<ColliderSet>,
    
) {
    for (entity, handle, mut transform) in query.iter_mut() {

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