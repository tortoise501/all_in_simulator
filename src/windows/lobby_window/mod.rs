use std::f32::consts::PI;

use bevy::{a11y::AccessibilityNode, color::palettes::css::{DARK_GRAY, LIME, RED}, prelude::*, ui::widget::NodeImageMode};
use systems::ui_backend;

use crate::GameState;

use super::{gen_generic_button, gen_generic_button_text};

mod systems;

pub struct Lobby;

impl Plugin for Lobby {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Lobby), systems::startup::setup_menu)
        .add_systems(Update, ui_backend::update_lobby_info.run_if(in_state(GameState::Lobby)));

    }
}

#[derive(Component)]
struct PlayerList;
