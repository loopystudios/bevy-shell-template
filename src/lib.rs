use bevy::{prelude::*, window::WindowMode};

pub const LAUNCHER_TITLE: &str = "Bevy Shell - Template";

pub fn app(fullscreen: bool) -> App {
    let mode = if fullscreen {
        WindowMode::BorderlessFullscreen
    } else {
        WindowMode::Windowed
    };
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: LAUNCHER_TITLE.to_string(),
        canvas: Some("#bevy".to_string()),
        fit_canvas_to_parent: true,
        mode,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(load_icon);
    app
}

fn load_icon(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("bevy.png"),
        ..default()
    });
}
