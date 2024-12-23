use std::f32::consts::PI;

use bevy::{a11y::AccessibilityNode, color::palettes::css::{DARK_GRAY, LIME, RED}, prelude::*, ui::widget::NodeImageMode};

pub struct JoinMenu;

impl Plugin for JoinMenu {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .add_systems(Startup, setup_menu)
        // .add_systems(Update, button_system)
        ;

    }
}

#[derive(Component)]
enum ButtonType {
    Join,
    Exit,
}
#[derive(Component)]
enum InputType {
    IPAddress,
    Port,
    Password,
}


fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn((Camera2d, IsDefaultUiCamera, UiBoxShadowSamples(6)));

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
            parent.spawn((gen_generic_button(),ButtonType::Join)).with_child(gen_generic_text("Join".to_string()));
            parent.spawn((gen_generic_button(),ButtonType::Exit)).with_child(gen_generic_text("Exit".to_string()));
        })
        ;
}



use super::gen_generic_button;
use super::gen_generic_text;