use std::{net::{SocketAddr, UdpSocket}, str::Bytes, thread::sleep, time::{Duration, SystemTime}};

use bevy::{prelude::*};
use bevy_renet2::{netcode::NetcodeServerPlugin, prelude::{ConnectionConfig, DefaultChannel, RenetServer, RenetServerPlugin, ServerEvent}};
use renet2_netcode::{NativeSocket, NetcodeServerTransport, ServerAuthentication, ServerSetupConfig};
use serde::{Deserialize, Serialize, Serializer};

use crate::{global_events::{CreateServer, SendServerMessage, UpdateLobby}, networking::{client, ClientMessages, ServerMessages}, GameState};

use super::HostState;

const PROTOCOL_ID: u64 = 7;
pub struct ServerPlugin;
impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenetServerPlugin);
        app.add_plugins(NetcodeServerPlugin);
        app.insert_resource(LobbyInfo{players:Vec::new()});
        app.add_systems(OnEnter(HostState::Server), create_server);
        app.add_systems(OnExit(HostState::Server), stop_server);
        app.add_systems(Update, before_create_server);
        app.add_systems(
            Update, 
            (send_message_system,receive_message_system,handle_events_system).run_if(in_state(HostState::Server).and(resource_exists::<RenetServer>))
        );
    }
}


fn new_renet_server(public_addr:SocketAddr) -> (RenetServer, NetcodeServerTransport) {
    // let public_addr = "127.0.0.1:10501".parse().unwrap();
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

fn before_create_server(
    ev_create_server:EventReader<CreateServer>,
    mut next_host_state: ResMut<NextState<HostState>>
) {
    if !ev_create_server.is_empty(){
        next_host_state.set(HostState::Server);
    }
}

fn create_server(
    mut ev_create_server:EventReader<CreateServer>,
    mut commands:Commands,
    mut lobby_info: ResMut<LobbyInfo>,
) {
    info!("crating server - function");
    for ev in ev_create_server.read() {
        lobby_info.players.push(LobbyPlayer{ id: 0, name: "Host".to_string() });
        info!("crating server - event");
        let (client, transport) = new_renet_server(ev.0);
    
        commands.insert_resource(client);
        commands.insert_resource(transport);
        info!("crating server - complete");
    }
}

fn stop_server(
    mut commands:Commands
) {
    commands.remove_resource::<RenetServer>();
    commands.remove_resource::<NetcodeServerTransport>();
}



fn send_message_system(
    mut server: ResMut<RenetServer>,
    mut ev_send_server_messages: EventReader<SendServerMessage>,
    mut ev_update_lobby_player_list: EventWriter<UpdateLobby>
) {
    let channel_id = 0;
    // Send a text message for all clients
    // The enum DefaultChannel describe the channels used by the default configuration
    for ev in ev_send_server_messages.read() {
        server.broadcast_message(DefaultChannel::ReliableOrdered, bincode::serialize(&ev.0).unwrap());
        match &ev.0 {
            ServerMessages::UpdateLobbyData(lobby_info) => {ev_update_lobby_player_list.send(UpdateLobby(lobby_info.clone()));},
            _ => (),
        }
    }
    // server.broadcast_message(DefaultChannel::ReliableOrdered, bincode::serialize(&ServerMessages::Test).unwrap());
}

fn receive_message_system(
    mut server: ResMut<RenetServer>,
    mut lobby_info: ResMut<LobbyInfo>,
    mut ev_send_server_messages: EventWriter<SendServerMessage>,
) {
    // Receive message from all clients
    for client_id in server.clients_id() {
        // while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
        //     info!("server received:{:?}",message)
        //     // Handle received message
        // }
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
            // Handle received message
            info!("received message:{:?}",message);
            match bincode::deserialize::<ClientMessages>(&message).unwrap() {
                ClientMessages::SendName(name) => {
                    lobby_info.players.push(LobbyPlayer { id: client_id, name: name });
                    // ev_update_lobby_player_list.send(UpdateLobby(lobby_info.clone()));
                    ev_send_server_messages.send(SendServerMessage(ServerMessages::UpdateLobbyData(lobby_info.clone())));
                },
                // ClientMessages::Test => todo!(),
                _ => todo!(),
            }
        }
    }
    
}

fn handle_events_system(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
    mut ev_send_server_messages: EventWriter<SendServerMessage>,
    mut lobby_info: ResMut<LobbyInfo>,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                info!("Client {client_id} connected");
                server.send_message(*client_id, DefaultChannel::ReliableOrdered, bincode::serialize(&ServerMessages::ConfirmConnection).unwrap());
                ev_send_server_messages.send(SendServerMessage(ServerMessages::UpdateLobbyData(lobby_info.clone())));
                // server.broadcast_message(DefaultChannel::ReliableOrdered, bincode::serialize(&ServerMessages::UpdateLobbyData(crate::LobbyInfo { player_names: clients_vec })).unwrap());// temp for lobby player list
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("Client {client_id} disconnected: {reason}");
            }
        }
    }
}




#[derive(Resource, Clone,Serialize,Deserialize)]
pub struct LobbyInfo {
    pub players: Vec<LobbyPlayer>
}


#[derive(Clone,Serialize,Deserialize)]
pub struct LobbyPlayer {
    pub id:u64,
    pub name:String,
}