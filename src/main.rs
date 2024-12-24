mod windows;
// mod windows::main_menu;
use bevy::prelude::*;
fn main() {
    let _app = App::new().add_plugins(windows::join_window::JoinMenu).run();
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum GameState {
    MainMenu,
    LobbyList,
    Lobby,
    InGame,
}

