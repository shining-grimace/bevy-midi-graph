use crate::states::AppState;
use bevy::{prelude::*, render::view::RenderLayers};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), init_loading_hud)
            .add_systems(OnExit(AppState::Loading), remove_loading_hud)
            .add_systems(OnEnter(AppState::Splash), init_splash_hud)
            .add_systems(OnExit(AppState::Splash), remove_splash_hud)
            .add_systems(OnEnter(AppState::Game), init_game_hud);
    }
}

#[derive(Component)]
struct UiRoot;

fn init_loading_hud(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands
        .spawn((
            UiRoot,
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                align_items: AlignItems::End,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Node::DEFAULT
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    padding: UiRect::all(Val::Px(16.0)),
                    ..default()
                },
                Text("Loading...".to_owned()),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE.into()),
                TextLayout {
                    justify: JustifyText::Center,
                    ..default()
                },
            ));
        });
}

fn remove_loading_hud(
    mut commands: Commands,
    ui_query: Query<Entity, With<UiRoot>>,
    camera_query: Query<Entity, With<Camera>>,
) {
    let ui = ui_query.get_single().unwrap();
    let camera = camera_query.get_single().unwrap();
    commands.entity(ui).despawn_recursive();
    commands.entity(camera).despawn_recursive();
}

fn init_splash_hud(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Camera {
            clear_color: ClearColorConfig::None,
            order: 1,
            ..default()
        },
        RenderLayers::layer(1),
    ));
    commands
        .spawn((
            UiRoot,
            RenderLayers::layer(1),
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Start,
                width: Val::Percent(50.0),
                height: Val::Percent(100.0),
                padding: UiRect {
                    left: Val::Percent(50.0),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    padding: UiRect::all(Val::Px(16.0)),
                    ..default()
                },
                Text("ACORN GAME".to_owned()),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.1).into()),
                TextLayout {
                    justify: JustifyText::Center,

                    ..default()
                },
            ));
        });
}

fn remove_splash_hud(
    mut commands: Commands,
    ui_query: Query<Entity, With<UiRoot>>,
    camera_query: Query<Entity, With<Camera2d>>,
) {
    let ui = ui_query.get_single().unwrap();
    commands.entity(ui).despawn_recursive();

    let camera = camera_query.get_single().unwrap();
    commands.entity(camera).despawn_recursive();
}

fn init_game_hud(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Camera {
            clear_color: ClearColorConfig::None,
            order: 1,
            ..default()
        },
        RenderLayers::layer(1),
    ));
    commands
        .spawn((
            UiRoot,
            RenderLayers::layer(1),
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::End,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    padding: UiRect::all(Val::Px(32.0)),
                    ..default()
                },
                Text("Score: 0".to_owned()),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.6).into()),
                TextLayout {
                    justify: JustifyText::Center,

                    ..default()
                },
            ));
        });
}
