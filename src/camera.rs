use bevy::prelude::*;
use bevy::prelude::MouseButton;
use bevy::prelude::KeyCode;
use bevy_third_person_camera::ThirdPersonCamera;
use bevy_third_person_camera::Zoom;
use bevy_third_person_camera::Offset;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 5.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ThirdPersonCamera {
            aim_enabled: true,
            aim_button: MouseButton::Right,
            aim_speed: 3.0,
            aim_zoom: 0.7,
            cursor_lock_toggle_enabled: true,
            cursor_lock_active: true,
            cursor_lock_key: KeyCode::Space,
            mouse_sensitivity: 1.0,
            mouse_orbit_button_enabled: false,
            mouse_orbit_button: MouseButton::Middle,
            offset_enabled: true,
            offset: Offset::new(0.5, 0.4),
            zoom: Zoom::new(1.5, 30.0),
            zoom_sensitivity: 1.0,
            ..default()
        }
    );
    commands.spawn(camera);
}
