use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};

use bevy::prelude::*;
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