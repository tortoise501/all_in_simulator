use bevy::{prelude::*};
use bevy_renet2::netcode::{
    ClientAuthentication, NativeSocket, NetcodeClientPlugin, NetcodeClientTransport, NetcodeServerPlugin, NetcodeServerTransport,
    NetcodeTransportError, ServerAuthentication, ServerSetupConfig,
};
use bevy_renet2::prelude::{
    client_connected, ClientId, ConnectionConfig, DefaultChannel, RenetClient, RenetClientPlugin, RenetServer, RenetServerPlugin,
    ServerEvent,
};

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::time::SystemTime;
use std::{collections::HashMap, net::UdpSocket};

use serde::{Deserialize, Serialize};

use crate::global_events::{ConnectToServer, UpdateLobby};
use crate::networking::ServerMessages;
use crate::GameState;

use super::HostState;

const PROTOCOL_ID: u64 = 7;
pub struct ClientPlugin;
impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenetClientPlugin);
        app.add_plugins(NetcodeClientPlugin);
        app.add_systems(OnEnter(HostState::Client), create_connection);
        app.add_systems(OnExit(HostState::Client), stop_client);
        app.add_systems(Update, before_create_connection);
        app.add_systems(Update, (send_message_system,receive_message_system).run_if(in_state(HostState::Client).and(resource_exists::<RenetClient>)));
    }
}

fn new_renet_client(server_addr: SocketAddr) -> (RenetClient, NetcodeClientTransport) {
    // let server_addr = "127.0.0.1:5000".parse().unwrap();
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

    let transport = NetcodeClientTransport::new(current_time, authentication, NativeSocket::new(socket).unwrap()).unwrap();//TODO IDK BAD UNWRAPS
    let client = RenetClient::new(ConnectionConfig::test(), transport.is_reliable());

    (client, transport)
}

fn create_connection(
    mut ev_connect_to_server:EventReader<ConnectToServer>,
    mut commands:Commands,
) {
    info!("crating client - function");
    for ev in ev_connect_to_server.read() {
        info!("crating client - event");
        let (client, transport) = new_renet_client(ev.0.socket);
    
        commands.insert_resource(client);
        commands.insert_resource(transport);
        info!("crating client - complete");
    }
}

fn before_create_connection(
    ev_connect_to_server:EventReader<ConnectToServer>,
    mut next_host_state: ResMut<NextState<HostState>>
) {
    if !ev_connect_to_server.is_empty(){
        next_host_state.set(HostState::Client);
    }
}

fn stop_client(
    mut commands:Commands
) {
    commands.remove_resource::<RenetClient>();
    commands.remove_resource::<NetcodeClientTransport>();
}

fn send_message_system(mut client: ResMut<RenetClient>) {
    // Send a text message to the server
    // client.send_message(DefaultChannel::ReliableOrdered, "client message");
}

fn receive_message_system(
    mut client: ResMut<RenetClient>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut ev_update_lobby_data: EventWriter<UpdateLobby>
) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        // Handle received message
        info!("received message:{:?}",message);
        match bincode::deserialize::<ServerMessages>(&message).unwrap() {
            ServerMessages::ConfirmConnection => {
                info!("confirming connection:{:?}",message);
                next_game_state.set(GameState::Lobby);
            },
            ServerMessages::Test => info!("test message:{:?}",message),
            ServerMessages::UpdateLobbyData(data) => {
                ev_update_lobby_data.send(UpdateLobby(data));
            }
            _ => todo!(),
        }
    }
}