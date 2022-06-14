use bevy::prelude::*;
use super::AppState;
pub struct LoadStatePlugin;
impl Plugin for LoadStatePlugin{
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(AppState::PreLoad)
                    .with_system(pre_load_to_menu))
            ;
    }
}

fn pre_load_to_menu(mut state: ResMut<State<AppState>>){
    state.set(AppState::Menu);
}

