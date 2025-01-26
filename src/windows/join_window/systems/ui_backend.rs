use bevy::{input::keyboard::{Key, KeyboardInput}, prelude::*};

use crate::networking::HostState;

use super::*;

pub fn input_system(
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

pub fn button_system (
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
    mut next_host_state: ResMut<NextState<HostState>>,
    mut ev_check_and_connect: EventWriter<events::ConnectTo>,
    input_ip: Res<IPInput>,
    input_port: Res<PortInput>,
    input_password: Res<PasswordInput>,
) {
    for (interaction, children, button_type) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match button_type {
                    InteractiveType::Join => {
                        info!("Join");
                        ev_check_and_connect.send(events::ConnectTo(input_ip.clone(),input_port.clone(),input_password.clone()));
                        next_host_state.set(HostState::Client);
                    },
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