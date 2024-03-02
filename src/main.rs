use bevy::prelude::*;
use bevy_turborand::{prelude::RngPlugin, DelegatedRng, GlobalRng};
use bevy_xpbd_3d::{
    math::{Quaternion, Vector},
    prelude::*,
};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        PhysicsPlugins::default(),
        RngPlugin::default(),
    ))
    .add_systems(Startup, (setup, spawn_cubes).chain())
    .add_systems(Update, cubes_jump);

    app.run();
}

#[derive(Component)]
struct JumpCooldown(Timer);

impl JumpCooldown {
    const MIN_COOLDOWN: f32 = 1.0;
    const MAX_COOLDOWN: f32 = 5.0;

    fn new(global_rng: &mut GlobalRng) -> Self {
        Self(Timer::from_seconds(
            global_rng.f32() * (Self::MAX_COOLDOWN - Self::MIN_COOLDOWN) + Self::MIN_COOLDOWN,
            TimerMode::Once,
        ))
    }

    fn new_timer(&mut self, global_rng: &mut GlobalRng) {
        self.0 = Timer::from_seconds(
            global_rng.f32() * (Self::MAX_COOLDOWN - Self::MIN_COOLDOWN) + Self::MIN_COOLDOWN,
            TimerMode::Once,
        );
    }
}

fn cubes_jump(
    mut cubes: Query<(&Transform, &mut LinearVelocity, &mut JumpCooldown), Without<JumpTarget>>,
    time: Res<Time>,
    mut global_rng: ResMut<GlobalRng>,
    target_cube: Query<&Transform, (With<JumpTarget>, Without<JumpCooldown>)>,
) {
    if let Ok(target_transform) = target_cube.get_single() {
        for (cube_transform, mut linear_velocity, mut jump_cooldown) in cubes.iter_mut() {
            if jump_cooldown.0.tick(time.delta()).just_finished() {
                jump_cooldown.new_timer(&mut global_rng);

                let direction = target_transform.translation - cube_transform.translation;
                let direction = direction.normalize();
                let max_speed = 5.0;
                // Randomize the jump force, between 1.0 and 10.0
                let jump_force = global_rng.f32() * 9.0 + 1.0;
                let mut desired_velocity = direction * max_speed;
                desired_velocity.y = jump_force;
                linear_velocity.0 = desired_velocity;
            }
        }
    }
}

#[derive(Component)]
struct JumpTarget;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Spawn ground and generate a collider for the mesh using AsyncCollider
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(80.0, 80.0)),
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
        transform: Transform::from_xyz(2.0, 20.0, 2.0),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-50.0, 30.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::from_size(Vec3::new(0.2, 0.2, 0.2)))),
            material: materials.add(Color::rgb_u8(0, 0, 255)),
            transform: Transform::from_xyz(-20.0, 2.0, 0.0),
            ..default()
        },
        JumpTarget,
    ));
}

fn spawn_cubes(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut global_rng: ResMut<GlobalRng>,
) {
    let cube_size = 1.0;
    let spacing_between_cubes = 0.1;
    let num_cubes = 5;
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
                            j as f32 * (cube_size + spacing_between_cubes) + 1.0,
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
                    JumpCooldown::new(&mut global_rng),
                    Friction::new(0.4),
                ));
            }
        }
    }
}
