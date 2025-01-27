use bevy::prelude::*;
use systems::lobby_list;

use crate::GameState;


mod systems;
mod ui_components;

pub struct Lobby;

impl Plugin for Lobby {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Lobby), systems::startup::setup_menu)
        .add_systems(Update, lobby_list::update_lobby_info.run_if(in_state(GameState::Lobby)));

    }
}

