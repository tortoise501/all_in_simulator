
use bevy::prelude::*;
use events::UpdateHelperText;

use super::*;

pub fn input_color_system(
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

pub fn helper_prompt_system(
    mut helper_text: Query<(&mut Text),With<HelperText>>,
    mut ev_helper_text: EventReader<UpdateHelperText>,
) {
    let mut text = helper_text.single_mut();
    for ev in ev_helper_text.read() {
        text.0 = ev.0.clone();
    }
    // if text.0 != helper_text_promot.0 {
    //     text.0 = helper_text_promot.0.clone();
    // }
}