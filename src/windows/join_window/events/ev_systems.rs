use std::net::Ipv4Addr;

use crate::{global_events::ConnectToServer, networking::TargetLobbyData};

use super::*;
use bevy::prelude::*;

pub(crate) fn check_input_and_connect (
    mut ev_check_and_connect: EventReader<ConnectTo>,
    mut ev_connect_to_server: EventWriter<ConnectToServer>
) {
    for ev in ev_check_and_connect.read() {
        let target_lobby = TargetLobbyData {
            address: match ev.0.0.parse::<Ipv4Addr>() {
                Ok(addr) => addr,
                Err(err) => {
                    println!("{}",err);
                    return;
                },
            },
            port: match ev.1.0.parse::<u16>() {
                Ok(p) => p,
                Err(err) => {
                    println!("{}",err);
                    return;
                },
            },
            password: ev.2.0.clone(),
        };
        println!("Joining {:?}",target_lobby);
        //TODO connect to server
        ev_connect_to_server.send(ConnectToServer(target_lobby));
    }
}