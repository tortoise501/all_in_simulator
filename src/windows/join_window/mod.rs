use std::{f32::consts::PI, net::{IpAddr}, ops::Deref};

use bevy::{a11y::AccessibilityNode, color::palettes::css::{DARK_GRAY, LIME, RED}, input::{keyboard::{}}, prelude::*, text::TextWriter, ui::widget::NodeImageMode};
mod systems;
mod events;
pub struct JoinMenu;

impl Plugin for JoinMenu {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(crate::GameState::LobbyList), systems::startup::setup_system)
        .add_systems(Update, systems::ui_backend::input_system.run_if(in_state(crate::GameState::LobbyList)))
        .add_systems(Update, systems::ui_backend::button_system.run_if(in_state(crate::GameState::LobbyList)))
        .add_systems(Update, systems::ui_frontend::input_color_system.run_if(in_state(crate::GameState::LobbyList)))
        .add_systems(Update, events::ev_systems::check_input_and_connect.run_if(in_state(crate::GameState::LobbyList)))
        .add_systems(Update, systems::ui_frontend::helper_prompt_system.run_if(in_state(crate::GameState::LobbyList)))
        .add_event::<events::ConnectTo>()
        .add_event::<events::UpdateHelperText>()
        // .insert_resource(IPInput("".to_string()))
        // .insert_resource(PortInput("".to_string()))
        // .insert_resource(PasswordInput("".to_string()))
        .insert_resource(JoinInputs{ ip_addr: "".to_string(), port: "".to_string(), password: "".to_string(), name: "".to_string() })
        .insert_state(InputState::NotInput)
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
    NameInput
}

#[derive(Component)]
struct Input;


#[derive(Component)]
enum TextType {
    IP,
    Port,
    Password,
    Name,
}


#[derive(Component)]
struct HelperText;



// #[derive(Resource, Clone)]
// struct IPInput(String);

// #[derive(Resource, Clone)]
// struct PortInput(String);

// #[derive(Resource, Clone)]
// struct PasswordInput(String);

#[derive(Resource, Clone)]
struct JoinInputs {
    ip_addr:String,
    port:String,
    password:String,
    name:String
}






#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum InputState {
    #[default]
    NotInput,
    IP,
    Port,
    Password,
    Name
}
use super::gen_generic_button_text;