mod windows;
// mod windows::main_menu;
use bevy::prelude::*;
fn main() {
    let _app = App::new().add_plugins((DefaultPlugins,windows::main_menu_window::MainMenu,windows::join_window::JoinMenu)).insert_state(GameState::MainMenu).enable_state_scoped_entities::<GameState>().run();
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum GameState {
    MainMenu,
    LobbyList,
    Lobby,
    InGame,
}

