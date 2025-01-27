
use bevy::prelude::*;

use crate::windows::lobby_window::ui_components;

pub fn setup_menu(mut commands: Commands) {
    info!("Setting up lobby menu");
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
            parent.spawn(ui_components::lobby_list::lobby_list());
            parent.spawn(ui_components::lobby_comms::lobby_comms());
        });
}