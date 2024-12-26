use std::{f32::consts::PI, net::{IpAddr}, ops::Deref};

use bevy::{a11y::AccessibilityNode, color::palettes::css::{DARK_GRAY, LIME, RED}, input::{keyboard::{}}, prelude::*, text::TextWriter, ui::widget::NodeImageMode};
mod systems;
mod events;
pub struct JoinMenu;

impl Plugin for JoinMenu {
    fn build(&self, app: &mut App) {
        app
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .add_systems(OnEnter(crate::GameState::LobbyList), systems::startup::setup_system)
        .add_systems(Update, systems::ui_backend::input_system.run_if(in_state(crate::GameState::LobbyList)))
        .add_systems(Update, systems::ui_backend::button_system.run_if(in_state(crate::GameState::LobbyList)))
        .add_systems(Update, systems::ui_frontend::input_color_system)
        .add_systems(Update, events::ev_systems::check_input_and_connect)
        .add_event::<events::ConnectTo>()
        .insert_resource(IPInput("".to_string()))
        .insert_resource(PortInput("".to_string()))
        .insert_resource(PasswordInput("".to_string()))
        .insert_state(InputState::NotInput)
        // .add_systems(Update, button_system)
        ;

    }
}

#[derive(Component, PartialEq, Eq, Debug)]

enum InteractiveType {
    Join,
    Exit,
    IPAddressInput,
    PortInput,
    PasswordInput,
}

#[derive(Component)]
struct Input;

#[derive(Component)]
enum TextType {
    IP,
    Port,
    Password,
}




#[derive(Resource, Clone)]
struct IPInput(String);

#[derive(Resource, Clone)]
struct PortInput(String);

#[derive(Resource, Clone)]
struct PasswordInput(String);





#[derive(Debug)]
struct TargetLobbyData {
    address: IpAddr,
    port: i16,
    password: String,
}



#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum InputState {
    #[default]
    NotInput,
    IP,
    Port,
    Password,
}
use super::gen_generic_button_text;