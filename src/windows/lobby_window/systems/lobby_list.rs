use bevy::prelude::*;

use crate::{global_events::UpdateLobby, windows::gen_generic_description_text};

pub fn update_lobby_info(
    mut ev_update_lobby: EventReader<UpdateLobby>,
    query: Query<(Entity, &Name)>,
    mut commands: Commands,
) {
    if ev_update_lobby.is_empty() {
        return;
    }
    info!("Updating lobby visual information");
    let player_list = query
        .iter()
        .find(|(_, name)| name.as_str() == "Player List")
        .map(|(entity, _)| entity);
    
    if let Some(player_list) = player_list {
        commands.entity(player_list).despawn_descendants();
        for ev in ev_update_lobby.read() {
            for player_name in ev.0.players.clone() {
                let child = commands.spawn(gen_generic_description_text(player_name.name)).id();
                commands.entity(player_list).add_child(child);
            }
        }
    }
}