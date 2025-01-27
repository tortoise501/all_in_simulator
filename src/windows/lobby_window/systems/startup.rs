use std::f32::consts::PI;

use bevy::{a11y::AccessibilityNode, color::palettes::css::{DARK_GRAY, LIME, RED}, prelude::*, ui::widget::NodeImageMode};

use crate::windows::{gen_generic_button, gen_generic_button_text, gen_generic_description_text, lobby_window::PlayerList};

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("Body"),
            StateScoped(crate::GameState::Lobby),
            Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            row_gap: Val::Px(20.),
            border: UiRect { left: Val::Px(10.), right: Val::Px(10.), top: Val::Px(10.), bottom: Val::Px(10.) },
            ..default()
        },
        BorderColor(Color::BLACK),
    ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Player List"),
                Node {
                    width: Val::Percent(30.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.),
                    border: UiRect { left: Val::Px(5.), right: Val::Px(5.), top: Val::Px(5.), bottom: Val::Px(5.) },
                    ..default()
                },
                BorderColor(Color::linear_rgb(200., 150., 150.)),
                // PlayerList,
            )).with_children(|parent|{
                parent.spawn(gen_generic_description_text("1111".to_string()));
                parent.spawn(gen_generic_description_text("2222".to_string()));
                parent.spawn(gen_generic_description_text("3333".to_string()));
            });
            parent.spawn((
                Name::new("Lobby Comms"),
                Node {
                    width: Val::Percent(70.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.),
                    border: UiRect { left: Val::Px(5.), right: Val::Px(5.), top: Val::Px(5.), bottom: Val::Px(5.) },
                    ..default()
                },
                BorderColor(Color::linear_rgb(150., 200., 150.)),
            ));
        });
}