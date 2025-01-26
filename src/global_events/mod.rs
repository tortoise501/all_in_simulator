use std::net::SocketAddr;

use bevy::prelude::Event;

use crate::networking::TargetLobbyData;

mod ev_systems;
#[derive(Event)]
pub struct ConnectToServer(pub TargetLobbyData);


#[derive(Event)]
pub struct CreateServer(pub SocketAddr);