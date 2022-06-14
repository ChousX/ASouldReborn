use bevy::prelude::*;
use crate::share::AppState;

pub struct CameraPlugin;
impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::PreLoad)
                    .with_system(add_ui_camera)
                    .with_system(add_2d_camera))
            ;
    }
}

fn add_ui_camera(mut commands: Commands){

}

fn add_2d_camera(mut commands: Commands){

}
