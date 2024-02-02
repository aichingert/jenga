use bevy::prelude::*;

#[derive(Component)]
struct MyCamera {
    x: f32,
    y: f32,
    z: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_cube)
        .add_systems(Update, move_camera)
        .run()
}

fn move_camera(
    keys: Res<Input<KeyCode>>,
    mut camera: Query<(&mut MyCamera, &mut Transform)>,
) {
    let (mut values, mut transform) = camera.get_single_mut().unwrap();

    if keys.pressed(KeyCode::W) {
        values.x += 0.001;
        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_x(values.x));
    }
    if keys.pressed(KeyCode::S) {
        values.x -= 0.001;
        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_x(values.x));
    }

    if keys.pressed(KeyCode::A) {
        values.y += 0.001;
        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(values.y));
    } 
    if keys.pressed(KeyCode::D) {
        values.y -= 0.001;
        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(values.y));
    }
}

fn spawn_camera(
    mut commands: Commands,
) { 
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10., 10., 20.)
                .looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
        },
        MyCamera { x: 0., y: 0., z: 0. },
    ));
}

fn spawn_cube(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
) {
    let shape = mesh.add(shape::Cube::default().into());

    let data = std::fs::read_to_string("data").unwrap();

    for line in data.lines() {
        let (src, dst) = line.split_once("~").unwrap();

        let src_xyz = src.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let dst_xyz = dst.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();

        for x in src_xyz[0]..=dst_xyz[0] {
            for y in src_xyz[1]..=dst_xyz[1] {
                for z in src_xyz[2]..=dst_xyz[2] {
                    commands.spawn((
                        PbrBundle {
                            mesh: shape.clone(),
                            transform: Transform::from_xyz(x as f32, z as f32, y as f32),
                            ..default()
                        },
                    ));
                }
            }
        }
    }
}
