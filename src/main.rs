use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_systems((setup, spawn_cubes))
        .add_system(camera_movement)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // flying camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    let mut camera = query.single_mut();
    let mut forward = camera.forward();
    forward.y = 0.;
    forward = forward.normalize();
    let speed = 3.;
    let rotate_speed = 3.;
    if keyboard_input.pressed(KeyCode::W) {
        camera.translation += forward * speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::S) {
        camera.translation -= forward * speed * time.delta_seconds();
    }
    let right = camera.rotation * Vec3::X;
    if keyboard_input.pressed(KeyCode::D) {
        camera.translation += right * speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::A) {
        camera.translation -= right * speed * time.delta_seconds();
    }
    let up = camera.rotation * Vec3::Y;
    if keyboard_input.pressed(KeyCode::Space) {
        camera.translation += up * speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::LShift) {
        camera.translation -= up * speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds());
    }
    if keyboard_input.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds());
    }
}
fn spawn_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = generate_cube(0., 0., 0., 1);
    let mesh2 = generate_cube(0., 2., 0., 1);
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_xyz(0.0, 1., 0.0),
    //     ..default()
    // });

    let material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: material,
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh2),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
}

/**
 * Generates a cube mesh with the given position and length
* the position is the center of the cube
* length is the side length of the cube
 */
fn generate_cube(x: f32, y: f32, z: f32, length: usize) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    // let mut normals = Vec::new();
    let mut indices = Vec::new();
    // let mut uvs = Vec::new();

    let half_length = length as f32 / 2.0;
    let vertices = vec![
        // front
        [x - half_length, y - half_length, z + half_length],
        [x + half_length, y - half_length, z + half_length],
        [x + half_length, y + half_length, z + half_length],
        [x - half_length, y + half_length, z + half_length],
        // back
        [x - half_length, y - half_length, z - half_length],
        [x + half_length, y - half_length, z - half_length],
        [x + half_length, y + half_length, z - half_length],
        [x - half_length, y + half_length, z - half_length],
    ];

    indices.extend_from_slice(&[
        0, 1, 2, 2, 3, 0, // front
        1, 5, 6, 6, 2, 1, // right
        7, 6, 5, 5, 4, 7, // back
        4, 0, 3, 3, 7, 4, // left
        4, 5, 1, 1, 0, 4, // bottom
        3, 2, 6, 6, 7, 3, // top
    ]);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.duplicate_vertices();
    mesh.compute_flat_normals();
    // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh
}
