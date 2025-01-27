use std::net::SocketAddr;

use bevy::prelude::Event;

use crate::{networking::{ClientMessages, ServerMessages, TargetLobbyData,server::LobbyInfo}};

mod ev_systems;
#[derive(Event)]
pub struct ConnectToServer(pub TargetLobbyData);


#[derive(Event)]
pub struct CreateServer(pub SocketAddr);


#[derive(Event)]
pub struct UpdateLobby(pub LobbyInfo);

#[derive(Event)]
pub struct SendServerMessage(pub ServerMessages);

#[derive(Event)]
pub struct SendClientMessage(pub ClientMessages);