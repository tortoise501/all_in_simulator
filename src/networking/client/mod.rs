use bevy::{prelude::*};
use bevy_renet2::netcode::{
    ClientAuthentication, NativeSocket, NetcodeClientPlugin, NetcodeClientTransport, NetcodeServerPlugin, NetcodeServerTransport,
    NetcodeTransportError, ServerAuthentication, ServerSetupConfig,
};
use bevy_renet2::prelude::{
    client_connected, ClientId, ConnectionConfig, DefaultChannel, RenetClient, RenetClientPlugin, RenetServer, RenetServerPlugin,
    ServerEvent,
};

use std::time::SystemTime;
use std::{collections::HashMap, net::UdpSocket};

use serde::{Deserialize, Serialize};

const PROTOCOL_ID: u64 = 7;
pub struct ClientPlugin;
impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
    }
}

fn new_renet_client() -> (RenetClient, NetcodeClientTransport) {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        socket_id: 0,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, NativeSocket::new(socket).unwrap()).unwrap();
    let client = RenetClient::new(ConnectionConfig::test(), transport.is_reliable());

    (client, transport)
}

// fn create_connection(mut ev_connect_to_server:EventReader<ConnectToServer>, mut commands:Commands) {
//     for ev in ev_connect_to_server.read() {
//         let authentication = ClientAuthentication::Unsecure {
//             server_addr: std::net::SocketAddr::V4(SocketAddrV4::new(ev.0.address, ev.0.port)),
//             client_id: 0,
//             user_data: None,
//             protocol_id: 0,
//             socket_id: todo!(),
//         };
//         let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
//         let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
//         let mut transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    
//         commands.insert_resource(transport);
//     }
// }

// fn send_message_system(mut client: ResMut<RenetClient>) {
//     // Send a text message to the server
//     client.send_message(DefaultChannel::ReliableOrdered, "server message");
// }

// fn receive_message_system(mut client: ResMut<RenetClient>) {
//     while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
//         // Handle received message
//     }
// }