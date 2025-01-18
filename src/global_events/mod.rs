use bevy::prelude::Event;

use crate::networking::TargetLobbyData;

mod ev_systems;
#[derive(Event)]
pub struct ConnectToServer(pub TargetLobbyData);