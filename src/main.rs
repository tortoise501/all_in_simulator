mod windows;
use windows::main_menu_window;
// mod windows::main_menu;
use bevy::prelude::*;
fn main() {
    let _app = App::new().add_plugins(main_menu_window::MainMenu).run();
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum GameState {
    MainMenu,
    LobbyList,
    Lobby,
    InGame,
}

