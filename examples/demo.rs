use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_midi_graph::{
    GraphAssetLoader, LoopFileSource, MidiFileSource, MidiGraph, MidiGraphAudioContext,
    MidiGraphPlugin, OneShotFileSource, Sf2FileSource,
};
use midi_graph::{EventChannel, NodeControlEvent, NodeEvent};

const PLAYER_VELOCITY: f32 = 3.0;

const MIDI_CONFIG: &str = "demo/graph.ron";
const PROGRAM_NO: usize = 1;
const MIDI_NODE_ID: u64 = 101;
const DEFAULT_ANCHOR: u32 = 0;
const ENTER_TENSION_ANCHOR: u32 = 1;

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct GraphAssetLoading {
    pub asset_handle: Handle<MidiGraph>,
}

pub fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default(), MidiGraphPlugin))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1000.0,
        })
        .add_systems(Startup, (initialise_assets, set_up_ui).chain())
        .add_systems(Update, (move_character, check_graph_ready))
        .add_systems(PostUpdate, check_intersections)
        .run();
}

fn initialise_assets(world: &mut World) {
    let asset_server = world.resource::<AssetServer>();
    let resource = GraphAssetLoading {
        asset_handle: asset_server.load(MIDI_CONFIG),
    };
    world.insert_resource(resource);
}

fn set_up_ui(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
}

fn check_graph_ready(
    server: Res<AssetServer>,
    mut audio_context: ResMut<MidiGraphAudioContext>,
    asset_metadata: ResMut<GraphAssetLoading>,
    asset_server: Res<AssetServer>,
    graphs: ResMut<Assets<MidiGraph>>,
    midi_assets: Res<Assets<MidiFileSource>>,
    sf2_assets: Res<Assets<Sf2FileSource>>,
    loop_assets: Res<Assets<LoopFileSource>>,
    one_shot_assets: Res<Assets<OneShotFileSource>>,
    mut graph_did_start: Local<bool>,
) {
    if *graph_did_start {
        return;
    }
    if !server.is_loaded_with_dependencies(asset_metadata.asset_handle.id()) {
        return;
    }
    let loader = GraphAssetLoader::new(
        &asset_server,
        &midi_assets,
        &sf2_assets,
        &loop_assets,
        &one_shot_assets,
    );
    *graph_did_start = true;
    let asset = graphs.get(&asset_metadata.asset_handle).unwrap();
    let program_existed = audio_context
        .store_new_program(PROGRAM_NO, &asset.config, &loader)
        .unwrap();
    if program_existed {
        panic!("Unexpectedly stored a program in an existing slot");
    }
    audio_context.change_program(PROGRAM_NO).unwrap();
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
    asset_metadata: Res<GraphAssetLoading>,
    mut audio_context: ResMut<MidiGraphAudioContext>,
    player_query: Query<Entity, With<Player>>,
    sensor_query: Query<Entity, With<Sensor>>,
    mut collision_started_events: EventReader<CollisionStarted>,
    mut collision_ended_events: EventReader<CollisionEnded>,
    mut graphs: ResMut<Assets<MidiGraph>>,
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
        let graph_id = asset_metadata.asset_handle.id();
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
