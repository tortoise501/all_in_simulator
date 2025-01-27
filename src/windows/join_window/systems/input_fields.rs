use bevy::{input::keyboard::{Key, KeyboardInput}, prelude::*};

use super::*;

/// Changes text in input fields
pub fn input_text_system (
    mut evr_kbd: EventReader<KeyboardInput>,
    mut inputs: ResMut<JoinInputs>,
    input_state: Res<State<SelectedInputField>>,
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
                    SelectedInputField::NotInput => {},
                    SelectedInputField::Input(input_field_type) => {
                        match input_field_type {
                            InputFieldType::IP => inputs.ip_addr.push_str(&input),
                            InputFieldType::Port => inputs.port.push_str(&input),
                            InputFieldType::Password => inputs.password.push_str(&input),
                            InputFieldType::Name => inputs.name.push_str(&input),
                        }
                    },
                }
            },
            Key::Backspace => {
                _ = match input_state.get() {
                    SelectedInputField::NotInput => {},
                    SelectedInputField::Input(input_field_type) => {
                        match input_field_type {
                            InputFieldType::IP => {inputs.ip_addr.pop();},
                            InputFieldType::Port => {inputs.port.pop();},
                            InputFieldType::Password => {inputs.password.pop();},
                            InputFieldType::Name => {inputs.name.pop();},
                        }
                    },
                };
            }
            _ => {}
        }
        for (mut text, text_type) in &mut text_q {
            match text_type {
                TextType::IP => text.0 = inputs.ip_addr.to_string(),
                TextType::Port => text.0 = inputs.port.to_string(),
                TextType::Password => text.0 = String::from_utf8(vec![b'*'; inputs.password.len()]).unwrap(),
                TextType::Name => text.0 = inputs.name.to_string(),
            }
        }
    }
}

/// Selects clicked input fields
pub fn input_field_selection_system (
    mut interaction_query: Query<
        (
            &Interaction,
            &Children,
            &InputFieldType,
        ),
        Changed<Interaction>,
    >,
    input_state: Res<State<SelectedInputField>>,
    mut next_input_state: ResMut<NextState<SelectedInputField>>,
) {
    for (interaction, _children, pressed_input_field_type) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                info!("pressed");
                match input_state.get() {
                    SelectedInputField::NotInput => next_input_state.set(SelectedInputField::Input(pressed_input_field_type.clone())),
                    SelectedInputField::Input(selected_input_field_type) => {
                        if pressed_input_field_type != selected_input_field_type{
                            next_input_state.set(SelectedInputField::Input(pressed_input_field_type.clone()));
                        }
                    },
                }
            }
            Interaction::Hovered => {
            }
            Interaction::None => {
            }
        }
    }
}

/// Changes color of selected input fields
pub fn input_color_system(
    selected_input: Res<State<SelectedInputField>>,
    mut input_fields_q: Query<
    (
        &mut BorderColor,
        &Children,
        &InputFieldType
    ),
    With<InputField>,>
) {
    for (mut border_color, _children, pressed_input_field_type) in &mut input_fields_q {
        match selected_input.get() {
            SelectedInputField::NotInput => border_color.0 = Color::linear_rgb(0., 0., 0.),
            SelectedInputField::Input(selected_input_field_type) => {
                if pressed_input_field_type == selected_input_field_type {
                    border_color.0 = Color::linear_rgb(0.5, 0.5, 0.5)
                } else {
                    border_color.0 = Color::linear_rgb(0., 0., 0.)
                }
            },
        }
    }
}