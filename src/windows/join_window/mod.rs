use std::{f32::consts::PI, net::{IpAddr, Ipv4Addr}, ops::Deref};

use bevy::{a11y::AccessibilityNode, color::palettes::css::{DARK_GRAY, LIME, RED}, input::{keyboard::{Key, KeyboardInput}, ButtonState}, prelude::*, text::TextWriter, ui::widget::NodeImageMode};

pub struct JoinMenu;

impl Plugin for JoinMenu {
    fn build(&self, app: &mut App) {
        app
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .add_systems(OnEnter(crate::GameState::LobbyList), setup_menu)
        .add_systems(Update, input_system.run_if(in_state(crate::GameState::LobbyList)))
        .add_systems(Update, button_system.run_if(in_state(crate::GameState::LobbyList)))
        .add_systems(Update, input_color_system)
        .add_systems(Update, check_input_and_connect)
        .add_event::<ConnectTo>()
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


fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {

    // root node
    commands
        .spawn((
            Name::new("Lobby UI"),
            StateScoped(crate::GameState::LobbyList),
            Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.),
            ..default()
        }))
        .with_children(|parent| {
            //Ip input field
            parent.spawn(gen_generic_node()).with_child(gen_generic_description_text("Enter IP of your computer you want to join.".to_string()));
            parent.spawn((
                Input,
                Button,
                InteractiveType::IPAddressInput,
                Node {
                    width: Val::Px(500.0),
                    height: Val::Px(50.0),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::BLACK),
                BackgroundColor(Color::linear_rgb(100., 250., 200.)),
        
            )).with_child(
                (
                    TextType::IP,
                    Text::new(""),
                    TextFont {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0., 0., 0.)),
                )
            );

            //Port input field
            parent.spawn(gen_generic_node()).with_child(gen_generic_description_text("Enter network Port that server uses.".to_string()));
            parent.spawn((
                Input,
                Button,
                InteractiveType::PortInput,
                Node {
                    width: Val::Px(500.0),
                    height: Val::Px(50.0),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::BLACK),
                BackgroundColor(Color::linear_rgb(100., 250., 200.)),
        
            )).with_child(
                (
                    TextType::Port,
                    Text::new(""),
                    TextFont {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0., 0., 0.)),
                )
            );

            //Password input field
            parent.spawn(gen_generic_node()).with_child(gen_generic_description_text("Enter lobby password".to_string()));
            parent.spawn((
                Input,
                Button,
                InteractiveType::PasswordInput,
                Node {
                    width: Val::Px(500.0),
                    height: Val::Px(50.0),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::BLACK),
                BackgroundColor(Color::linear_rgb(100., 250., 200.)),
        
            )).with_child(
                (
                    TextType::Password,
                    Text::new(""),
                    TextFont {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0., 0., 0.)),
                )
            );


            parent.spawn((gen_generic_button(),InteractiveType::Join)).with_child(gen_generic_button_text("Join".to_string()));
            parent.spawn((gen_generic_button(),InteractiveType::Exit)).with_child(gen_generic_button_text("Exit".to_string()));
        })
        ;
}

fn input_system(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut input_ip: ResMut<IPInput>,
    mut input_port: ResMut<PortInput>,
    mut input_password: ResMut<PasswordInput>,
    input_state: Res<State<InputState>>,
    mut text_q: Query<(&mut Text, &TextType)>,
) {
    for ev in evr_kbd.read() {
        // We don't care about key releases, only key presses
        if ev.state == ButtonState::Released {
            continue;
        }
        match &ev.logical_key {
            // Handle key presses that produce text characters
            Key::Character(input) => {
                // Ignore any input that contains control (special) characters
                if input.chars().any(|c| c.is_control()) {
                    continue;
                }
                match input_state.get() {
                    InputState::NotInput => {},
                    InputState::IP => input_ip.0.push_str(&input),
                    InputState::Port => input_port.0.push_str(&input),
                    InputState::Password => input_password.0.push_str(&input),
                }
            },
            Key::Backspace => {
                _ = match input_state.get() {
                    InputState::NotInput => {},
                    InputState::IP => {input_ip.0.pop();},
                    InputState::Port => {input_port.0.pop();},
                    InputState::Password => {input_password.0.pop();},
                };
            }
            _ => {}
        }
        for (mut text, text_type) in &mut text_q {
            match text_type {
                TextType::IP => text.0 = input_ip.0.to_string(),
                TextType::Port => text.0 = input_port.0.to_string(),
                TextType::Password => text.0 = String::from_utf8(vec![b'*'; input_password.0.len()]).unwrap(),
            }
        }
    }
}

fn input_color_system(
    selected_input: Res<State<InputState>>,
    mut input_fields_q: Query<
    (
        &mut BorderColor,
        &Children,
        &InteractiveType
    ),
    (With<Input>),>
) {
    for (mut border_color, children, button_type) in &mut input_fields_q {
        match selected_input.get() {
            InputState::IP if *button_type == InteractiveType::IPAddressInput => {border_color.0 = Color::linear_rgb(0.5, 0.5, 0.5)},
            InputState::Port if *button_type == InteractiveType::PortInput => {border_color.0 = Color::linear_rgb(0.5, 0.5, 0.5)},
            InputState::Password if *button_type == InteractiveType::PasswordInput => {border_color.0 = Color::linear_rgb(0.5, 0.5, 0.5)},
            _ => border_color.0 = Color::BLACK,
        }
    }
}





fn button_system (
    mut interaction_query: Query<
        (
            &Interaction,
            &Children,
            &InteractiveType,
        ),
        (Changed<Interaction>),
    >,
    input_state: Res<State<InputState>>,
    mut next_input_state: ResMut<NextState<InputState>>,
    mut next_game_state: ResMut<NextState<crate::GameState>>,
    mut ev_check_and_connect: EventWriter<ConnectTo>,
    input_ip: Res<IPInput>,
    input_port: Res<PortInput>,
    input_password: Res<PasswordInput>,
) {
    for (interaction, children, button_type) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                println!("pressed {:?} state is {:?}",button_type,input_state.get());
                match button_type {
                    InteractiveType::Join => {ev_check_and_connect.send(ConnectTo(input_ip.clone(),input_port.clone(),input_password.clone()));},
                    InteractiveType::Exit => {
                        next_game_state.set(crate::GameState::MainMenu);
                    },
                    InteractiveType::IPAddressInput if *input_state.get() != InputState::IP => next_input_state.set(InputState::IP),
                    InteractiveType::PortInput if *input_state.get() != InputState::Port => next_input_state.set(InputState::Port),
                    InteractiveType::PasswordInput if *input_state.get() != InputState::Password => next_input_state.set(InputState::Password),
                    _ => {}
                }
            }
            Interaction::Hovered => {
            }
            Interaction::None => {
            }
        }
    }
}

#[derive(Resource, Clone)]
struct IPInput(String);

#[derive(Resource, Clone)]
struct PortInput(String);

#[derive(Resource, Clone)]
struct PasswordInput(String);



fn check_input_and_connect (
    mut ev_check_and_connect: EventReader<ConnectTo>
) {
    for ev in ev_check_and_connect.read() {
        let target_lobby = TargetLobbyData {
            address: match ev.0.0.parse::<Ipv4Addr>() {
                Ok(addr) => std::net::IpAddr::V4(addr),
                Err(err) => {
                    println!("{}",err);
                    return;
                },
            },
            port: match ev.1.0.parse::<i16>() {
                Ok(p) => p,
                Err(err) => {
                    println!("{}",err);
                    return;
                },
            },
            password: ev.2.0.clone(),
        };
        println!("{:?}",target_lobby);
        //TODO connect to server
    }
}

#[derive(Event)]
struct ConnectTo(IPInput,PortInput,PasswordInput);


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

use super::{gen_generic_button, gen_generic_description_text, gen_generic_node};
use super::gen_generic_button_text;