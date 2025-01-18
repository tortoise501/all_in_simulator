use std::{net::UdpSocket, time::SystemTime};

use bevy::{prelude::*};
use bevy_renet2::{netcode::NetcodeServerPlugin, prelude::{ConnectionConfig, DefaultChannel, RenetServer, RenetServerPlugin, ServerEvent}};
use renet2_netcode::{NativeSocket, NetcodeServerTransport, ServerAuthentication, ServerSetupConfig};

use crate::GameState;

use super::HostState;

const PROTOCOL_ID: u64 = 7;
pub struct ServerPlugin;
impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenetServerPlugin);
        app.add_plugins(NetcodeServerPlugin);
        let (server,transport) = new_renet_server();
        app.insert_resource(server);
        app.insert_resource(transport);
        app.add_systems(
            Update, 
            (send_message_system,receive_message_system,handle_events_system).run_if(in_state(HostState::Server))
        );
    }
}


fn new_renet_server() -> (RenetServer, NetcodeServerTransport) {
    let public_addr = "127.0.0.1:10501".parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let server_config = ServerSetupConfig {
        current_time,
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        socket_addresses: vec![vec![public_addr]],
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(server_config, NativeSocket::new(socket).unwrap()).unwrap();
    let server = RenetServer::new(ConnectionConfig::test());

    (server, transport)
}



fn send_message_system(mut server: ResMut<RenetServer>) {
    let channel_id = 0;
    // Send a text message for all clients
    // The enum DefaultChannel describe the channels used by the default configuration
    server.broadcast_message(DefaultChannel::ReliableOrdered, "server message");
}

fn receive_message_system(mut server: ResMut<RenetServer>) {
    // Receive message from all clients
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
            // Handle received message
        }
    }
}

fn handle_events_system(mut server_events: EventReader<ServerEvent>) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {client_id} connected");
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {client_id} disconnected: {reason}");
            }
        }
    }
}