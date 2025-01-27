
use bevy::prelude::*;
use events::UpdateHelperText;

use super::*;



pub fn helper_prompt_system(
    mut helper_text: Query<&mut Text,With<HelperText>>,
    mut ev_helper_text: EventReader<UpdateHelperText>,
) {
    let mut text = helper_text.single_mut();
    for ev in ev_helper_text.read() {
        text.0 = ev.0.clone();
        info!("Helper prompt changed");
    }
}