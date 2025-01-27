use bevy::prelude::*;
use super::*;

pub fn setup_system(mut commands: Commands) {
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
            parent.spawn(ui_components::input_field::input_field_box(InputFieldType::IP))
            .with_child(
                ui_components::input_field::input_field_text(TextType::IP)
            );

            //Port input field
            parent.spawn(gen_generic_node()).with_child(gen_generic_description_text("Enter network Port that server uses.".to_string()));
            parent.spawn(ui_components::input_field::input_field_box(InputFieldType::Port))
            .with_child(
                ui_components::input_field::input_field_text(TextType::Port)
            );

            //Password input field
            parent.spawn(gen_generic_node()).with_child(gen_generic_description_text("Enter lobby password".to_string()));
            parent.spawn(ui_components::input_field::input_field_box(InputFieldType::Password))
            .with_child(
                ui_components::input_field::input_field_text(TextType::Password)
            );

            parent.spawn(gen_generic_node()).with_child(gen_generic_description_text("Enter your username".to_string()));
            parent.spawn(ui_components::input_field::input_field_box(InputFieldType::Name))
            .with_child(
                ui_components::input_field::input_field_text(TextType::Name)
            );

            parent.spawn(gen_generic_node()).with_child((gen_generic_description_text("Enter data in all fields".to_string()),HelperText));



            parent.spawn((gen_generic_button(),ButtonType::Join)).with_child(gen_generic_button_text("Join".to_string()));
            parent.spawn((gen_generic_button(),ButtonType::Exit)).with_child(gen_generic_button_text("Exit".to_string()));
            
        })
        ;
}