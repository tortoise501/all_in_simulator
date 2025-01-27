use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use crate::{global_events::{ConnectToServer, SendClientMessage}, networking::TargetLobbyData};

use super::*;
use bevy::prelude::*;

pub(crate) fn check_input_and_connect (
    mut ev_check_and_connect: EventReader<ConnectTo>,
    mut ev_connect_to_server: EventWriter<ConnectToServer>,
    mut ev_helper_text: EventWriter<UpdateHelperText>,
    mut ev_send_messages: EventWriter<SendClientMessage>,
) {
    for ev in ev_check_and_connect.read() {
        let target_lobby = TargetLobbyData {
            // address: match ev.0.0.parse::<Ipv4Addr>() {
            //     Ok(addr) => addr,
            //     Err(err) => {
            //         println!("{}",err);
            //         return;
            //     },
            // },
            // port: match ev.1.0.parse::<u16>() {
            //     Ok(p) => p,
            //     Err(err) => {
            //         println!("{}",err);
            //         return;
            //     },
            // },
            socket: match format!("{}:{}",ev.0.ip_addr,ev.0.port).parse::<SocketAddr>() {
                Ok(socket) => socket,
                Err(err) => {
                            // println!("{}",err);
                            ev_helper_text.send(UpdateHelperText(err.to_string()));
                            return;
                        },
            },
            password: ev.0.password.clone(),
        };
        println!("Joining {:?}",target_lobby);
        ev_connect_to_server.send(ConnectToServer(target_lobby));
        ev_send_messages.send(SendClientMessage(crate::networking::ClientMessages::SendName(ev.0.name.clone())));
    }
}