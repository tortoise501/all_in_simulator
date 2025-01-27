use bevy::prelude::*;


use super::*;

pub fn button_system (
    mut interaction_query: Query<
        (
            &Interaction,
            &Children,
            &ButtonType,
        ),
        Changed<Interaction>,
    >,
    mut next_game_state: ResMut<NextState<crate::GameState>>,
    mut ev_check_and_connect: EventWriter<events::TryConnectionTo>,
    input: Res<JoinInputs>,
) {
    for (interaction, _children, button_type) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match button_type {
                    ButtonType::Join => {
                        info!("Join");
                        ev_check_and_connect.send(events::TryConnectionTo(input.clone()));
                    },
                    ButtonType::Exit => {
                        next_game_state.set(crate::GameState::MainMenu);
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