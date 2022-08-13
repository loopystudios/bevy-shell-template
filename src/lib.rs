use bevy::prelude::*;

const CLEAR_COLOR: ClearColor = ClearColor(Color::rgb(0.0, 0.0, 1.0));

pub fn app() -> App {
    let mut app = App::new();
    app.insert_resource(CLEAR_COLOR)
        .insert_resource(WindowDescriptor {
            title: "Bevy Shell - Template".to_string(),
            canvas: Some("#bevy".to_string()),
            fit_canvas_to_parent: true,
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
