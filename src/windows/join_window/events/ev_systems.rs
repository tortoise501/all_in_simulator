use std::net::SocketAddr;

use crate::{global_events::{ConnectToServer, SendClientMessage}, networking::TargetLobbyData};

use super::*;

pub(crate) fn check_input_and_connect (
    mut ev_check_and_connect: EventReader<TryConnectionTo>,
    mut ev_connect_to_server: EventWriter<ConnectToServer>,
    mut ev_helper_text: EventWriter<UpdateHelperText>,
    mut ev_send_messages: EventWriter<SendClientMessage>,
) {
    for ev in ev_check_and_connect.read() {
        let target_lobby = TargetLobbyData {
            socket: match format!("{}:{}",ev.0.ip_addr,ev.0.port).parse::<SocketAddr>() {
                Ok(socket) => socket,
                Err(err) => {
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