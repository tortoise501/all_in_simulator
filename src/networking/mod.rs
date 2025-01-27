use std::net::SocketAddr;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use server::LobbyInfo;

pub(super) mod client;
pub(super) mod server;


#[derive(Debug)]
pub struct TargetLobbyData {
    pub socket: SocketAddr,
    pub password: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States,Default)]
pub(super) enum HostState {
    #[default]
    NotPaying,
    Server,
    Client
}

#[derive(Serialize,Deserialize)]
pub enum ServerMessages {
    ConfirmConnection,
    Test,
    UpdateLobbyData(LobbyInfo)
}

#[derive(Serialize,Deserialize)]
pub enum ClientMessages {
    SendName(String),
    Test,
}