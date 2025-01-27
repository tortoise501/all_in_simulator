use bevy::prelude::*;

use crate::windows::join_window::{InputField, InputFieldType, TextType};

pub fn input_field_box(interactive_type:InputFieldType) -> impl Bundle {
    (
        InputField,
        Button,
        interactive_type,
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

    )
}

pub fn input_field_text(text_type:TextType) -> impl Bundle {
    (
        text_type,
        Text::new(""),
        TextFont {
            font_size: 33.0,
            ..default()
        },
        TextColor(Color::srgb(0., 0., 0.)),
    )
}