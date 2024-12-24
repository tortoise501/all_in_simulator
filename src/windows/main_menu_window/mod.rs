use std::f32::consts::PI;

use bevy::{a11y::AccessibilityNode, color::palettes::css::{DARK_GRAY, LIME, RED}, prelude::*, ui::widget::NodeImageMode};

pub struct MainMenu;

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .add_systems(OnEnter(crate::GameState::MainMenu), setup_menu.run_if(in_state(crate::GameState::MainMenu)))
        .add_systems(Update, button_system.run_if(in_state(crate::GameState::MainMenu)));

    }
}

#[derive(Component)]
enum ButtonType {
    Host,
    Join,
    Exit
}


fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn((Camera2d, IsDefaultUiCamera, UiBoxShadowSamples(6)));//TODO do it properly

    // root node
    commands
        .spawn((
            Name::new("Button"),
            StateScoped(crate::GameState::MainMenu),
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
            parent.spawn((gen_generic_button(),ButtonType::Host)).with_child(gen_generic_text("Host".to_string()));
            parent.spawn((gen_generic_button(),ButtonType::Join)).with_child(gen_generic_text("Join".to_string()));
            parent.spawn((gen_generic_button(),ButtonType::Exit)).with_child(gen_generic_text("Exit".to_string()));
        })
        ;
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &ButtonType,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_game_state: ResMut<NextState<crate::GameState>> 
) {
    for (interaction, mut color, mut border_color, children, button_type) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
                match button_type {
                    ButtonType::Host => {
                        println!("HOST");
                        next_game_state.set(crate::GameState::Lobby);//TODO HOST GAME
                    },
                    ButtonType::Join => {
                        println!("JOIN");
                        next_game_state.set(crate::GameState::LobbyList);
                    },
                    ButtonType::Exit => {
                        println!("Exit");
                        todo!("Exiting")//TODO EXIT GAME
                    },
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}




const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);


use super::gen_generic_button;
use super::gen_generic_text;