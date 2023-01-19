use bevy::{prelude::*, window::WindowId, winit::WinitWindows};
use image;
use std::io::Cursor;
use winit::window::Icon;

fn set_window_icon(windows: NonSend<WinitWindows>) {
    let window = windows.get_window(WindowId::primary()).expect("no window");
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
    let mut app = game::app(true);

    info!("Starting launcher: iOS");
    app.add_startup_system(set_window_icon);
    app.run();
}
