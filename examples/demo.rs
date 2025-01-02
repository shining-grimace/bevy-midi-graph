use avian3d::prelude::*;
use bevy::{asset::LoadState, prelude::*};
use bevy_midi_graph::{MidiGraphAsset, MidiGraphAudioContext, MidiGraphPlugin};
use midi_graph::{EventChannel, NodeControlEvent, NodeEvent};

const PLAYER_VELOCITY: f32 = 3.0;

const PROGRAM_NO: usize = 1;
const MIDI_NODE_ID: u64 = 101;
const DEFAULT_ANCHOR: u32 = 0;
const ENTER_TENSION_ANCHOR: u32 = 1;

#[derive(Component)]
struct Player;

#[derive(Resource, Default)]
struct GraphAssetLoading(Handle<MidiGraphAsset>);

pub fn main() {
    App::new()
        .insert_resource(GraphAssetLoading::default())
        .add_plugins((DefaultPlugins, PhysicsPlugins::default(), MidiGraphPlugin))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1000.0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (move_character, check_graph_ready))
        .add_systems(PostUpdate, check_intersections)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut graph_asset: ResMut<GraphAssetLoading>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Collider::cuboid(20.0, 1.0, 20.0),
        RigidBody::Static,
        Mesh3d(meshes.add(Cuboid::new(20.0, 1.0, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.5, 0.2, 0.2),
            ..default()
        })),
        Transform::from_xyz(0.0, -0.5, 0.0),
    ));
    commands.spawn((
        Player,
        LinearVelocity::ZERO,
        Collider::cylinder(0.5, 2.0),
        RigidBody::Dynamic,
        Mesh3d(meshes.add(Cylinder::new(0.5, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.8),
            ..default()
        })),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));
    commands.spawn((
        Sensor,
        Collider::cuboid(6.0, 6.0, 6.0),
        Mesh3d(meshes.add(Cuboid::new(6.0, 6.0, 6.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.8, 0.3),
            ..default()
        })),
        Transform::from_xyz(-5.0, 3.0, -2.0),
    ));
    graph_asset.0 = asset_server.load("demo/graph.ron");
}

fn check_graph_ready(
    server: Res<AssetServer>,
    loading: Res<GraphAssetLoading>,
    assets: Res<Assets<MidiGraphAsset>>,
    mut mixer: ResMut<MidiGraphAudioContext>,
    mut graph_did_start: Local<bool>,
) {
    if *graph_did_start {
        return;
    }
    let load_state = server.get_load_state(loading.0.id()).unwrap();
    match load_state {
        LoadState::Failed(e) => panic!("{}", e),
        LoadState::Loaded => {
            *graph_did_start = true;
            let asset = assets.get(&loading.0).unwrap();
            let program_existed = mixer.store_new_program(PROGRAM_NO, &asset.config).unwrap();
            if program_existed {
                panic!("Unexpectedly stored a program in an existing slot");
            }
            mixer.change_program(PROGRAM_NO).unwrap();
        }
        _ => {}
    }
}

fn move_character(
    mut player_velocity_query: Query<&mut LinearVelocity, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut quit_signal: EventWriter<AppExit>,
) {
    let mut player_velocity = player_velocity_query.single_mut();
    let left = keyboard_input.pressed(KeyCode::ArrowLeft);
    let right = keyboard_input.pressed(KeyCode::ArrowRight);
    let up = keyboard_input.pressed(KeyCode::ArrowUp);
    let down = keyboard_input.pressed(KeyCode::ArrowDown);
    let quit = keyboard_input.pressed(KeyCode::Escape);

    let input_velocity = if up && left {
        Vec3::new(-0.707, 0.0, -0.707)
    } else if up && right {
        Vec3::new(0.707, 0.0, -0.707)
    } else if down && left {
        Vec3::new(-0.707, 0.0, 0.707)
    } else if down && right {
        Vec3::new(0.707, 0.0, 0.707)
    } else if left {
        Vec3::new(-1.0, 0.0, 0.0)
    } else if right {
        Vec3::new(1.0, 0.0, 0.0)
    } else if up {
        Vec3::new(0.0, 0.0, -1.0)
    } else if down {
        Vec3::new(0.0, 0.0, 1.0)
    } else {
        Vec3::ZERO
    };
    *player_velocity = LinearVelocity(PLAYER_VELOCITY * input_velocity);

    if quit {
        quit_signal.send(AppExit::Success);
    }
}

fn check_intersections(
    graph: Res<GraphAssetLoading>,
    mut audio_context: ResMut<MidiGraphAudioContext>,
    player_query: Query<Entity, With<Player>>,
    sensor_query: Query<Entity, With<Sensor>>,
    mut collision_started_events: EventReader<CollisionStarted>,
    mut collision_ended_events: EventReader<CollisionEnded>,
    mut graphs: ResMut<Assets<MidiGraphAsset>>,
    mut current_anchor: Local<u32>,
) {
    let player_entity = player_query.get_single().unwrap();
    let sensor_entity = sensor_query.get_single().unwrap();
    let started = collision_started_events.read().any(|event| {
        (event.0 == player_entity && event.1 == sensor_entity)
            || (event.0 == sensor_entity && event.1 == player_entity)
    });
    let ended = collision_ended_events.read().any(|event| {
        (event.0 == player_entity && event.1 == sensor_entity)
            || (event.0 == sensor_entity && event.1 == player_entity)
    });
    let desired_track = if started {
        ENTER_TENSION_ANCHOR
    } else if ended {
        DEFAULT_ANCHOR
    } else {
        return;
    };
    if *current_anchor != desired_track {
        *current_anchor = desired_track;
        let graph_id = graph.0.id();
        if let Some(graph) = graphs.get_mut(graph_id) {
            let channel: &mut EventChannel = audio_context
                .root_event_channel()
                .unwrap()
                .expect("No root event receiver on audio context");
            let send = channel.try_send(NodeEvent::NodeControl {
                node_id: MIDI_NODE_ID,
                event: NodeControlEvent::SeekWhenIdeal {
                    to_anchor: Some(desired_track),
                },
            });
            if let Err(err) = send {
                panic!("{:?}", err);
            }
        }
    }
}
