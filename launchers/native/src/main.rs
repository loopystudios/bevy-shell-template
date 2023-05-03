use bevy::{prelude::*, winit::WinitWindows};
use std::io::Cursor;
use bevy::window::PrimaryWindow;
use winit::window::Icon;

fn set_window_icon(
    primary_query: Query<Entity, With<PrimaryWindow>>,
    windows: NonSend<WinitWindows>,
) {
    let entity = primary_query.single();
    let window = windows
        .get_window(entity)
        .expect("should have primary window");
    let (icon_rgba, icon_width, icon_height) = {
        let icon_buf = Cursor::new(include_bytes!("../static/appicon.png"));
        let rgba = image::load(icon_buf, image::ImageFormat::Png)
            .expect("Failed to open icon path")
            .into_rgba8();

        let (width, height) = rgba.dimensions();
        let icon_raw = rgba.into_raw();
        (icon_raw, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height)
        .expect("error making icon");
    window.set_window_icon(Some(icon));
}

fn main() {
    let mut app = game::app(false);

    info!("Starting launcher: Native");
    app.add_startup_system(set_window_icon);
    app.run();
}
