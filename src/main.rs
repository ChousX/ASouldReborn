use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
    log::LogPlugin
};
mod share;
mod menu;
mod camera;
use share::{AppState, TITLE};

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        width: share::RESOLUTION.0,
        height: share::RESOLUTION.1,
        cursor_locked: false,
        cursor_visible: true,
        resizable: false,
        title: TITLE.to_string(),
        present_mode: PresentMode::Fifo,
        mode: WindowMode::BorderlessFullscreen,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(camera::CameraPlugin)
    .add_plugin(menu::MenuPlugin)
    .add_state(AppState::PreLoad)
    .run();
}

