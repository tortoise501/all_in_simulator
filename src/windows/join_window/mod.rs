use std::{f32::consts::PI, net::Ipv4Addr};

use bevy::{a11y::AccessibilityNode, color::palettes::css::{DARK_GRAY, LIME, RED}, input::{keyboard::{Key, KeyboardInput}, ButtonState}, prelude::*, text::TextWriter, ui::widget::NodeImageMode};

pub struct JoinMenu;

impl Plugin for JoinMenu {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .add_systems(Startup, setup_menu)
        .add_systems(Update, input_system)
        .add_systems(Update, button_system)
        .insert_resource(IPInput("".to_string()))
        .insert_state(InputState::NotInput)
        // .add_systems(Update, button_system)
        ;

    }
}

#[derive(Component)]
enum ButtonType {
    Join,
    Exit,
    IPAddressInput,
    PortInput,
    PasswordInput,
}

#[derive(Component)]
enum TextType {
    IP,
    Port,
    Password,
}


fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn((Camera2d, IsDefaultUiCamera, UiBoxShadowSamples(6)));

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
            parent.spawn((
                Button,
                ButtonType::IPAddressInput,
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
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                )
            );
            parent.spawn((gen_generic_button(),ButtonType::Join)).with_child(gen_generic_text("Join".to_string()));
            parent.spawn((gen_generic_button(),ButtonType::Exit)).with_child(gen_generic_text("Exit".to_string()));
        })
        ;
}

fn input_system(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut input_ip: ResMut<IPInput>,
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
                    InputState::NotInput => todo!(),
                    InputState::IP => input_ip.0.push_str(&input),
                    InputState::Port => todo!(),
                    InputState::Password => todo!(),
                }
            },
            Key::Backspace => {
                input_ip.0.pop();
            }
            _ => {}
        }
        for (mut text, text_type) in &mut text_q {
            match text_type {
                TextType::IP => text.0 = input_ip.0.to_string(),
                TextType::Port => todo!(),
                TextType::Password => todo!(),
            }
        }
    }
}


fn button_system (
    mut interaction_query: Query<
        (
            &Interaction,
            &Children,
            &ButtonType,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    input_state: Res<State<InputState>>,
    mut next_input_state: ResMut<NextState<InputState>>,
) {
    for (interaction, children, button_type) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match button_type {
                    ButtonType::Join => todo!(),
                    ButtonType::Exit => todo!(),
                    ButtonType::IPAddressInput if *input_state.get() != InputState::IP => next_input_state.set(InputState::IP),
                    ButtonType::PortInput if *input_state.get() != InputState::Port => next_input_state.set(InputState::Port),
                    ButtonType::PasswordInput if *input_state.get() != InputState::Password => next_input_state.set(InputState::Password),
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

#[derive(Resource)]
struct IPInput(String);

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum InputState {
    #[default]
    NotInput,
    IP,
    Port,
    Password,
}

use super::gen_generic_button;
use super::gen_generic_text;