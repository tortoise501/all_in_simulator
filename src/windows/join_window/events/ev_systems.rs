use std::net::Ipv4Addr;

use super::*;
use bevy::prelude::*;

pub(crate) fn check_input_and_connect (
    mut ev_check_and_connect: EventReader<ConnectTo>
) {
    for ev in ev_check_and_connect.read() {
        let target_lobby = TargetLobbyData {
            address: match ev.0.0.parse::<Ipv4Addr>() {
                Ok(addr) => std::net::IpAddr::V4(addr),
                Err(err) => {
                    println!("{}",err);
                    return;
                },
            },
            port: match ev.1.0.parse::<i16>() {
                Ok(p) => p,
                Err(err) => {
                    println!("{}",err);
                    return;
                },
            },
            password: ev.2.0.clone(),
        };
        println!("{:?}",target_lobby);
        //TODO connect to server
    }
}