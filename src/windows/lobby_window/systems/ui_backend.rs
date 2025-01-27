use bevy::{prelude::*, state::commands};

use crate::{global_events::UpdateLobby, windows::{gen_generic_description_text, lobby_window::PlayerList}};

pub fn update_lobby_info(
    mut ev_update_lobby: EventReader<UpdateLobby>,
    mut query: Query<(Entity, &Name)>,
    mut commands: Commands,
) {
    // Find the entity with the "Player List" name
    let player_list = query
        .iter()
        .find(|(_, name)| name.as_str() == "Player List")
        .map(|(entity, _)| entity);

    // If no matching entity is found, exit early
    if let Some(player_list) = player_list {
        if !ev_update_lobby.is_empty() {
            commands.entity(player_list).despawn_descendants();
        }

        // Iterate through the events and update the player list
        for ev in ev_update_lobby.read() {
            for player_name in ev.0.players.clone() {
                let child = commands.spawn(gen_generic_description_text(player_name.name)).id();
                commands.entity(player_list).add_child(child);
            }
        }
    }
}