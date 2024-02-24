use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

const BALL_SIZE: f32 = 5.;

const PADDLE_SPEED: f32 = 1.;
const PADDLE_WIDTH: f32 = 10.;
const PADDLE_HEIGHT: f32 = 50.;

#[derive(Component)]
struct Paddle;

#[derive(Bundle)]
struct PaddleBundle {
    paddle: Paddle,
    position: Position,
}

impl PaddleBundle {
    fn new(x: f32, y: f32) -> Self {
        PaddleBundle {
            paddle: Paddle,
            position: Position(Vec2::new(x, y)),
        }
    }
}

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Ball;

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    position: Position,
}

impl BallBundle {
    fn new() -> Self {
        BallBundle {
            ball: Ball,
            position: Position(Vec2::new(0., 0.)),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_ball, spawn_camera, spawn_paddles))
        .add_systems(Update, (move_ball, project_positions.after(move_ball)))
        .run();
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning ball...");

    let mesh = Mesh::from(Circle::new(BALL_SIZE));
    let material = ColorMaterial::from(Color::rgb(1., 0., 0.));

    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(material);

    commands.spawn((
        BallBundle::new(),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        },
    ));
}

fn move_ball(mut ball: Query<&mut Position, With<Ball>>) {
    if let Ok(mut position) = ball.get_single_mut() {
        position.0.x += 1.0
    }
}
fn project_positions(mut positionable: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionable {
        transform.translation = position.0.extend(0.);
    }
}
fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2dBundle::default());
}

fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning paddles...");

    let mesh = Mesh::from(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT));
    let material = ColorMaterial::from(Color::rgb(0., 1., 0.));

    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(material);

    commands.spawn((
        PaddleBundle::new(20., -25f32),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        },
    ));
}
