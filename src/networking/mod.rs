use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};

use bevy::prelude::*;
use bevy_renet2::prelude::Bytes;
use serde::{Deserialize, Serialize};

use crate::LobbyInfo;
pub(super) mod client;
pub(super) mod server;


struct TestChannel;


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