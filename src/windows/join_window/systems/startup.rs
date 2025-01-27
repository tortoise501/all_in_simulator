use bevy::prelude::*;
use super::*;

pub fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
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

            parent.spawn(gen_generic_node()).with_child(gen_generic_description_text("Enter your username".to_string()));
            parent.spawn((
                Input,
                Button,
                InteractiveType::NameInput,
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
                    TextType::Name,
                    Text::new(""),
                    TextFont {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0., 0., 0.)),
                )
            );

            parent.spawn(gen_generic_node()).with_child((gen_generic_description_text("Enter data in all fields".to_string()),HelperText));



            parent.spawn((gen_generic_button(),InteractiveType::Join)).with_child(gen_generic_button_text("Join".to_string()));
            parent.spawn((gen_generic_button(),InteractiveType::Exit)).with_child(gen_generic_button_text("Exit".to_string()));
            
        })
        ;
}