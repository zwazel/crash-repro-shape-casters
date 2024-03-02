use bevy::prelude::*;
use bevy_xpbd_3d::{
    math::{Quaternion, Vector},
    prelude::*,
};

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, (setup, spawn_cubes).chain());

    app.run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Spawn ground and generate a collider for the mesh using AsyncCollider
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(20.0, 20.0)),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        AsyncCollider(ComputedCollider::TriMesh),
        RigidBody::Static,
    ));

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1_000_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 8.0, 2.0),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-10.0, 3.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_cubes(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let cube_size = 1.0;
    let spacing_between_cubes = 0.1;
    let num_cubes = 10;
    let collider = Collider::cuboid(1.0, 1.0, 1.0);
    let mut caster_shape = collider.clone();
    caster_shape.set_scale(Vector::ONE * 0.99, 10);

    for i in 0..num_cubes {
        for j in 0..num_cubes {
            for k in 0..num_cubes {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::from_size(Vec3::new(
                            cube_size, cube_size, cube_size,
                        )))),
                        material: materials.add(Color::rgb_u8(124, 144, 255)),
                        transform: Transform::from_xyz(
                            i as f32 * (cube_size + spacing_between_cubes),
                            j as f32 * (cube_size + spacing_between_cubes) + 2.0,
                            k as f32 * (cube_size + spacing_between_cubes),
                        ),
                        ..default()
                    },
                    ShapeCaster::new(
                        caster_shape.clone(),
                        Vector::ZERO,
                        Quaternion::default(),
                        Direction3d::NEG_Y,
                    ),
                    RigidBody::Dynamic,
                    Collider::cuboid(1.0, 1.0, 1.0),
                ));
            }
        }
    }
}
