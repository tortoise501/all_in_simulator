use bevy::prelude::*;

pub fn lobby_list() -> impl Bundle{
    (
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
    )
}