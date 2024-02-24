use bevy::{math::bounding::*, prelude::*, sprite::MaterialMesh2dBundle};

const BALL_SIZE: f32 = 5.;
const BALL_SPEED: f32 = 5.;

const PADDLE_SPEED: f32 = 1.;
const PADDLE_WIDTH: f32 = 10.;
const PADDLE_HEIGHT: f32 = 50.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_ball, spawn_camera, spawn_paddles))
        .add_systems(Update, (move_ball, project_positions.after(move_ball)))
        .run();
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Shape(Vec2);

#[derive(Bundle)]
struct PaddleBundle {
    paddle: Paddle,
    shape: Shape,
    velocity: Velocity,
    position: Position,
}

impl PaddleBundle {
    fn new(x: f32, y: f32) -> Self {
        PaddleBundle {
            paddle: Paddle,
            shape: Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            velocity: Velocity(Vec2::new(0., 0.)),
            position: Position(Vec2::new(x, y)),
        }
    }
}

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    shape: Shape,
    velocity: Velocity,
    position: Position,
}

impl BallBundle {
    fn new(x: f32, y: f32) -> Self {
        BallBundle {
            ball: Ball,
            shape: Shape(Vec2::new(BALL_SIZE, BALL_SIZE)),
            velocity: Velocity(Vec2::new(x, y)),
            position: Position(Vec2::new(0., 0.)),
        }
    }
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
        BallBundle::new(1., 0.),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..default()
        },
    ));
}

fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0
    }
}
fn project_positions(mut ball: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut ball {
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

fn handle_collisions(
    mut ball: Query<(&mut Velocity, &Position, &Shape), With<Ball>>,
    other_things: Query<(&Position, &Shape), Without<Ball>>,
) {
    if let Ok((mut ball_velocity, ball_position, ball_shape)) = ball.get_single_mut() {
        for (position, shape) in &other_things {
            let collision = Aabb2d::new(ball_position.0, ball_shape.0)
                .intersects(&Aabb2d::new(position.0, shape.0));
        }
    }
}
