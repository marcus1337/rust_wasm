use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowPlugin, WindowTheme};

pub fn initialize_app(app: &mut App, canvas_id: Option<String>) {
    app.insert_resource(AssetMetaCheck::Never);
    initialize_app_plugins(app, canvas_id);
    app.add_systems(Startup, setup);
    app.insert_resource(ClearColor(Color::rgb(0.3, 0.2, 0.2)));
}

fn initialize_app_plugins(app: &mut App, canvas_id: Option<String>) {
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "My_Bevy_Window!".into(),
            canvas: canvas_id,
            resolution: (1920., 1080.).into(),
            present_mode: PresentMode::AutoVsync,
            window_theme: Some(WindowTheme::Dark),
            resizable: true,
            ..default()
        }),
        ..default()
    }));
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut _materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Hello Wörld",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 60.0,
                color: Color::WHITE,
            },
        ),
        ..Default::default()
    });
}
