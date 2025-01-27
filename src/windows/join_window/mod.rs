
use bevy::prelude::*;
mod systems;
mod events;
mod ui_components;
pub struct JoinMenu;

impl Plugin for JoinMenu {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(crate::GameState::LobbyList), systems::startup::setup_system)
        .add_systems(Update, systems::input_fields::input_text_system.run_if(in_state(crate::GameState::LobbyList)))
        .add_systems(Update, systems::buttons::button_system.run_if(in_state(crate::GameState::LobbyList)))
        .add_systems(Update, systems::input_fields::input_field_selection_system.run_if(in_state(crate::GameState::LobbyList)))
        .add_systems(Update, systems::input_fields::input_color_system.run_if(in_state(crate::GameState::LobbyList)))
        .add_systems(Update, events::ev_systems::check_input_and_connect.run_if(in_state(crate::GameState::LobbyList)))
        .add_systems(Update, systems::helper_prompt::helper_prompt_system.run_if(in_state(crate::GameState::LobbyList)))
        .add_event::<events::TryConnectionTo>()
        .add_event::<events::UpdateHelperText>()
        .insert_resource(JoinInputs{ ip_addr: "".to_string(), port: "".to_string(), password: "".to_string(), name: "".to_string() })
        .insert_state(SelectedInputField::NotInput)
        ;

    }
}


#[derive(Component,PartialEq, Eq,Debug)]
enum ButtonType {
    Join,
    Exit,
}

#[derive(Component,PartialEq, Eq,Debug,Clone,Hash)]
enum InputFieldType {
    IP,
    Port,
    Password,
    Name,
}

/// Marks input fields
#[derive(Component)]
struct InputField;

/// Marks for texts that display input value
#[derive(Component)]
enum TextType {
    IP,
    Port,
    Password,
    Name,
}

/// Text that shows helpful information about correctness of input values
#[derive(Component)]
struct HelperText;


/// Struct with all information given in form
#[derive(Resource, Clone)]
struct JoinInputs {
    ip_addr:String,
    port:String,
    password:String,
    name:String
}

/// Currently selected input field
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum SelectedInputField {
    #[default]
    NotInput,
    Input(InputFieldType),
}
use super::gen_generic_button_text;