use std::net::{IpAddr, Ipv4Addr};

use bevy::prelude::*;
pub(super) mod client;
pub(super) mod server;


struct TestChannel;


#[derive(Debug)]
pub struct TargetLobbyData {
    pub address: Ipv4Addr,
    pub port: u16,
    pub password: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States,Default)]
pub(super) enum HostState {
    #[default]
    NotPaying,
    Server,
    Client
}