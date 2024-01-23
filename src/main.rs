use bevy::{ prelude::*, DefaultPlugins};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;
use camera::CameraPlugin;
use player::PlayerPlugin;
use sheep::SheepPlugin;
use world::WorldPlugin;
// use sheep::SheepPlugin;

mod camera;
mod player;
mod world;
mod sheep;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Game devin til im in heaven".into(),
                    resolution: (2560., 1440.).into(),
                    ..Default::default()
                    }),
                ..Default::default()
                }),
            PlayerPlugin,
            WorldPlugin,
            CameraPlugin,
            WorldInspectorPlugin::new(),
            ThirdPersonCameraPlugin,
            SheepPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin {
                enabled: true,
                ..default()
            },
        ))
        .run();
}