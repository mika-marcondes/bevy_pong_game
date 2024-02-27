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
        .add_systems(
            Update,
            (
                move_ball,
                project_positions.after(move_ball),
                handle_collisions.after(move_ball),
            ),
        )
        .run();
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Component)]
struct Collider;

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
    collider: Collider,
}

impl PaddleBundle {
    fn new(x: f32, y: f32) -> Self {
        PaddleBundle {
            paddle: Paddle,
            shape: Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            velocity: Velocity(Vec2::new(0., 0.)),
            position: Position(Vec2::new(x, y)),
            collider: Collider,
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
    window: Query<&Window>,
) {
    println!("Spawning paddles...");

    if let Ok(window) = window.get_single() {
        let window_with = window.resolution.width();
        let padding = 50.;

        let right_paddle_x = window_with / 2. - padding;
        let left_paddle_x = -window_with / 2. + padding;

        let mesh = Mesh::from(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT));
        let material = ColorMaterial::from(Color::rgb(0., 1., 0.));

        let mesh_handle = meshes.add(mesh);
        let material_handle = materials.add(material);

        commands.spawn((
            PaddleBundle::new(right_paddle_x, 0.),
            MaterialMesh2dBundle {
                mesh: mesh_handle.clone().into(),
                material: material_handle.clone(),
                ..default()
            },
        ));

        commands.spawn((
            PaddleBundle::new(left_paddle_x, 0.),
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: material_handle,
                ..default()
            },
        ));
    }
}

fn handle_collisions(
    mut ball: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider: Query<&Transform, With<Collider>>,
) {
    if let Ok((mut ball_velocity, ball_transform)) = ball.get_single_mut() {
        for transform in &collider {
            let collision = collide_with_side(
                BoundingCircle::new(ball_transform.translation.truncate(), BALL_SIZE / 2.),
                Aabb2d::new(
                    transform.translation.truncate(),
                    transform.scale.truncate() / 2.,
                ),
            );

            if let Some(collision) = collision {
                println!("Smack that ball!!!11!");
                match collision {
                    Collision::Left => {
                        ball_velocity.0.x *= -1.;
                    }
                    Collision::Right => {
                        ball_velocity.0.x *= -1.;
                    }
                    Collision::Top => {
                        ball_velocity.0.y *= -1.;
                    }
                    Collision::Bottom => {
                        ball_velocity.0.y *= -1.;
                    }
                }
            }
        }
    }
}

fn collide_with_side(ball: BoundingCircle, paddle: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&paddle) {
        return None;
    }

    let closest = paddle.closest_point(ball.center());
    let offset = ball.center - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}
