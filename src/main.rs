mod windows;
mod networking;
mod global_events;
// mod windows::main_menu;
use bevy::prelude::*;
fn main() {
    let _app = App::new().add_plugins(
        (DefaultPlugins,
            windows::main_menu_window::MainMenu,
            windows::join_window::JoinMenu,
            windows::lobby_window::Lobby,
            networking::client::ClientPlugin,
            networking::server::ServerPlugin,
            global_events::ev_systems::GlobalEventPlugin,
        )
    )
    .add_systems(Startup, spawn_camera)
    .add_systems(Update, windows::button_color_system)
    .insert_state(GameState::MainMenu).enable_state_scoped_entities::<GameState>()
    .insert_state(networking::HostState::default())
    .run();
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
pub enum GameState {
    MainMenu,
    LobbyList,
    Lobby,
    InGame,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, IsDefaultUiCamera, UiBoxShadowSamples(6)));
}