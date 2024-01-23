use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct SheepPlugin;

impl Plugin for SheepPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_sheep)
           .add_systems(Update, sheep_movement);
    }
}

#[derive(Component)]
struct Sheep;

#[derive(Component)]
struct Speed(f32);

fn spawn_sheep(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut create_sheep = |color: Color, name: String, xyz: (f32, f32, f32)| -> (PbrBundle, Name, Sheep, Speed, Collider, RigidBody) {
        (
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule::default())),
                material: materials.add(color.into()),
                transform: Transform::from_xyz(xyz.0, xyz.1, xyz.2),
                ..Default::default()
            },
            Name::new(name),
            Sheep,
            Speed(1.0),
            Collider::cuboid(1.0 / 2.0, 2.0 / 2.0, 1.0 / 2.0),
            RigidBody::Dynamic,
        )
    };

    const NUM_SHEEP: usize = 10;

    for i in 0..NUM_SHEEP {
        let sheep_name = format!("sheep {}", i + 1);

        // Calculate the angle for the position in a circle
        let angle = 2.0 * std::f32::consts::PI * (i as f32) / (NUM_SHEEP as f32);

        // Calculate the position based on the angle and radius (adjust the radius as needed)
        let radius = 5.0;
        let x = radius * angle.cos();
        let z = radius * angle.sin();

        commands.spawn(create_sheep(
            Color::PINK,
            sheep_name,
            (x, 1.0, z),  // Adjust the third value if needed
        ));
    }
}


fn sheep_movement(
    time: Res<Time>,
    mut sheep_q: Query<(&mut Transform, &Speed), With<Sheep>>,
) {
    let delta_seconds = time.delta_seconds();
    for (mut transform, speed) in sheep_q.iter_mut() {
        
        // Calculate new position based on speed and elapsed time
        let new_x = transform.translation.x + speed.0 * delta_seconds;
        let new_z = transform.translation.z;

        // Update the sheep's position
        transform.translation.x = new_x;
        transform.translation.z = new_z;

        // Optional: Rotate the sheep based on movement direction
        let rotation = Quat::from_rotation_y(-delta_seconds * speed.0);
        transform.rotation *= rotation;
    }
}