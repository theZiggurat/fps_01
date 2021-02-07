mod systems;

use bevy::prelude::*;
use bevy::diagnostic::{
    DiagnosticsPlugin, FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_rapier3d::{
    rapier::dynamics::RigidBodyBuilder, 
    rapier::geometry::ColliderBuilder, 
    render::RapierRenderPlugin, 
    physics::RapierConfiguration
};

use bevy_mod_picking::*;

use systems::{
    FPSCameraPlugin, 
    PhysicsSystemPlugin
};

fn main() {
    App::build()
        .add_resource( WindowDescriptor {
            cursor_visible: false,
            cursor_locked: true,
            title: "FPS01".to_string(),
            width: 1000.0,
            height: 800.0,
            ..Default::default()
        })
        .add_resource(RapierConfiguration {
            time_dependent_number_of_timesteps: false,
            ..Default::default()
        })
        .add_resource(Msaa { samples: 8 } )
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsSystemPlugin)
        .add_startup_system(setup.system())
        .add_plugin(FPSCameraPlugin)
        //.add_plugin(RapierRenderPlugin)
        .add_plugin(EguiPlugin)
        // PickingPlugin provides core picking systems and must be registered first
        .add_plugin(PickingPlugin)
        // InteractablePickingPlugin adds mouse events and selection
        .add_plugin(InteractablePickingPlugin)
        // // DebugPickingPlugin systems to build and update debug cursors
        // .add_plugin(DebugPickingPlugin)
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_system(light_movement_system.system())
        .add_system(ui.system())
        .run();

    
}

fn ui(
    mut egui: ResMut<EguiContext>,
) {
    let ctx = &mut egui.ctx;
    egui::Window::new("Hello").show(ctx, |ui| {
        ui.label("world");

    });

}

fn light_movement_system(
    mut query: Query<&mut Transform, With<Light>>,
    time: Res<Time>
) {

    if let Some(mut transform) = query.iter_mut().nth(0) {

        let time: f32 = (time.seconds_since_startup() as f32) / 4.0;
        transform.translation = Vec3::new(time.sin() * 4.0, 8.0, time.cos() * 4.0);

        transform.look_at(Vec3::default(), Vec3::unit_y());
    }

}


fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {

    match asset_server.load_folder("models") {
        Ok(vec) => {
            vec.iter().for_each(|h| println!("{:?}", h));
        }
        Err(e) => println!("{}", e)
    }
    let mesh_handle = asset_server.load("models\\ball.gltf#Mesh0/Primitive0");
    //let mesh_mat = asset_server.load("models\\ball.gltf#Material0/Material.001");
    
    let material_handle = materials.add(StandardMaterial {
        albedo: Color::rgb(0.2, 0.8, 0.6),
        ..Default::default()
    });

    let floor_width = 100.0;
    let spawn = Vec3::new(0.0, 10.0, 0.0);

    commands
        .spawn(
        PbrBundle {
            mesh: mesh_handle,
            material: material_handle.clone(),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new_dynamic().translation(5.0, 10.0, 5.0))
        .with(ColliderBuilder::ball(4.0))
        // spawn floor
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            transform: Transform::from_scale(Vec3::new(floor_width*2.0, 1.0, floor_width*2.0)),
            material: material_handle.clone(),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new_static().translation(0.0, 0.0, 0.0))
        .with(ColliderBuilder::cuboid(floor_width, 0.5, floor_width).friction(0.5))
        // spawn cube
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 4.0})),
            material: material_handle,
            transform: Transform::from_matrix(Mat4::from_translation(spawn)),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new_dynamic().translation(spawn.x, spawn.y, spawn.z))
        .with(ColliderBuilder::cuboid(2.0, 2.0, 2.0).density(30.0))
        //.with(PickableBundle::default())

        // spawn light
        .spawn(LightBundle {
            transform: Transform::from_matrix(Mat4::from_translation(Vec3::new(4.0, 8.0, 4.0))),
            ..Default::default()
        });
        


        
}