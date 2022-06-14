use bevy::prelude::*;
use crate::share::AppState;

pub struct MenuPlugin;
impl Plugin for MenuPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::Menu)
                    .with_system(spawn_main_menu)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Menu)
                    .with_system(clean_up_main_menu)
            )
        ;
    }
}

fn spawn_main_menu(){}
fn clean_up_main_menu(){}