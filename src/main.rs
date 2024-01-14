use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::{WindowMode, WindowResized};
use bevy_rapier2d::prelude::*;

const SIZE: f32 = 1024.0;
const MARGIN: f32 = 128.0;

use bevy::render::camera::ScalingMode;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(256.0))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, player_movement_system)
        .add_systems(Update, on_resize_system)
        .add_systems(Update, change_color)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Component, Default)]
/// player component
struct Player {
    /// force exerted, N
    forward_force: f32,
    /// torque exterted, N.m
    max_torque: f32,
    colored: bool,
}

#[derive(Component, Default)]
/// interactive ball component
struct Ball {
    colored: bool,
}

#[derive(Resource)]
struct ColorHandles {
    player_white: Handle<Image>,
    player_red: Handle<Image>,
    ball_white: Handle<ColorMaterial>,
    ball_red: Handle<ColorMaterial>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();
    window.mode = WindowMode::BorderlessFullscreen;

    let player_white = asset_server.load("player_white.png");
    let player_red = asset_server.load("player_red.png");

    let mut bundle = Camera2dBundle::default();
    bundle.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: SIZE,
        min_height: SIZE,
    };
    commands.spawn(bundle);

    let white = materials.add(ColorMaterial::from(Color::rgb_u8(255, 255, 255)));
    let red = materials.add(ColorMaterial::from(Color::rgb_u8(255, 0, 0)));

    commands.insert_resource(ColorHandles {
        player_white: player_white.clone(),
        player_red,
        ball_white: white.clone(),
        ball_red: red,
    });

    commands.spawn((
        SpriteBundle {
            texture: player_white,
            ..default()
        },
        Player {
            max_torque: 0.02,
            forward_force: 50.0,
            colored: false,
        },
        RigidBody::Dynamic,
        Collider::ball(64.0),
        Restitution::coefficient(1.0),
        ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        },
        // never sleep
        Sleeping {
            angular_threshold: -1.0,
            linear_threshold: -1.0,
            sleeping: false,
        },
    ));

    let mesh: Mesh2dHandle = meshes.add(shape::Circle::new(16.0).into()).into();

    for pos in (-360..=360).step_by(36) {
        commands.spawn_batch(vec![
            (
                MaterialMesh2dBundle {
                    mesh: mesh.clone(),
                    material: white.clone(),
                    transform: Transform::from_xyz(pos as f32, 384.0, 0.0),
                    ..default()
                },
                Ball { colored: false },
                RigidBody::Dynamic,
                Collider::ball(16.0),
                Restitution::coefficient(1.0),
                ActiveEvents::COLLISION_EVENTS,
                // never sleep
                Sleeping {
                    angular_threshold: -1.0,
                    linear_threshold: -1.0,
                    sleeping: false,
                },
            ),
            (
                MaterialMesh2dBundle {
                    mesh: mesh.clone(),
                    material: white.clone(),
                    transform: Transform::from_xyz(pos as f32, -384.0, 0.0),
                    ..default()
                },
                Ball { colored: false },
                RigidBody::Dynamic,
                Collider::ball(16.0),
                Restitution::coefficient(1.0),
                ActiveEvents::COLLISION_EVENTS,
                // never sleep
                Sleeping {
                    angular_threshold: -1.0,
                    linear_threshold: -1.0,
                    sleeping: false,
                },
            ),
            (
                MaterialMesh2dBundle {
                    mesh: mesh.clone(),
                    material: white.clone(),
                    transform: Transform::from_xyz(384.0, pos as f32, 0.0),
                    ..default()
                },
                Ball { colored: false },
                RigidBody::Dynamic,
                Collider::ball(16.0),
                Restitution::coefficient(1.0),
                ActiveEvents::COLLISION_EVENTS,
                // never sleep
                Sleeping {
                    angular_threshold: -1.0,
                    linear_threshold: -1.0,
                    sleeping: false,
                },
            ),
            (
                MaterialMesh2dBundle {
                    mesh: mesh.clone(),
                    material: white.clone(),
                    transform: Transform::from_xyz(-384.0, pos as f32, 0.0),
                    ..default()
                },
                Ball { colored: false },
                RigidBody::Dynamic,
                Collider::ball(16.0),
                Restitution::coefficient(1.0),
                ActiveEvents::COLLISION_EVENTS,
                // never sleep
                Sleeping {
                    angular_threshold: -1.0,
                    linear_threshold: -1.0,
                    sleeping: false,
                },
            ),
        ]);
    }

    commands.spawn((
        Collider::compound(vec![
            //top
            (
                Vec2::new(0.0, (SIZE / 2.0) + (MARGIN / 2.0)),
                0.0,
                Collider::cuboid(SIZE / 2.0, MARGIN / 2.0),
            ),
            //bottom
            (
                Vec2::new(0.0, -((SIZE / 2.0) + (MARGIN / 2.0))),
                0.0,
                Collider::cuboid(SIZE / 2.0, MARGIN / 2.0),
            ),
            //left
            (
                Vec2::new(-((SIZE / 2.0) + (MARGIN / 2.0)), 0.0),
                0.0,
                Collider::cuboid(MARGIN / 2.0, (SIZE / 2.0) + MARGIN),
            ),
            //right
            (
                Vec2::new((SIZE / 2.0) + (MARGIN / 2.0), 0.0),
                0.0,
                Collider::cuboid(MARGIN / 2.0, (SIZE / 2.0) + MARGIN),
            ),
        ]),
        Restitution::coefficient(0.9),
        TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
    ));

    // Background
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb_u8(43, 44, 47),
            custom_size: Some(Vec2::splat(SIZE)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
        ..default()
    });
}

fn on_resize_system(
    mut windows: Query<&mut Window>,
    mut resize_events: EventReader<WindowResized>,
) {
    for e in resize_events.read() {
        windows.single_mut().resolution.set(e.width, e.height);
    }
}

fn player_movement_system(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform, &mut ExternalForce)>,
) {
    let (player, transform, mut force) = query.single_mut();

    let rotation_factor = match (
        input.pressed(KeyCode::Left) || input.pressed(KeyCode::A),
        input.pressed(KeyCode::Right) || input.pressed(KeyCode::D),
    ) {
        (true, false) => 1.0,
        (false, true) => -1.0,
        _ => 0.0,
    };

    let forward_factor = match (
        input.pressed(KeyCode::Up) || input.pressed(KeyCode::W),
        input.pressed(KeyCode::Down) || input.pressed(KeyCode::S),
    ) {
        (true, false) => 1.0,
        (false, true) => -1.0,
        _ => 0.0,
    };

    // apply force to front of object
    force.force = (transform.rotation * Vec3::Y * forward_factor * player.forward_force).truncate();
    force.torque = rotation_factor * player.max_torque;
}

fn change_color(
    mut events: EventReader<CollisionEvent>,
    mut player_query: Query<(&mut Handle<Image>, &mut Player)>,
    mut ball_query: Query<(&mut Handle<ColorMaterial>, &mut Ball)>,
    handles: Res<ColorHandles>,
) {
    for event in events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            for e in [e1, e2] {
                if let Ok((mut handle, mut ball)) = ball_query.get_mut(*e) {
                    ball.colored = !ball.colored;
                    *handle = match ball.colored {
                        false => handles.ball_white.clone(),
                        true => handles.ball_red.clone(),
                    };
                } else if let Ok((mut handle, mut player)) = player_query.get_mut(*e) {
                    player.colored = !player.colored;
                    *handle = match player.colored {
                        false => handles.player_white.clone(),
                        true => handles.player_red.clone(),
                    };
                }
            }
        }
    }
}
