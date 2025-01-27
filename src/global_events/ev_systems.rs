use bevy::prelude::*;
use super::*;

pub struct GlobalEventPlugin;
impl Plugin for GlobalEventPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<ConnectToServer>()
        .add_event::<CreateServer>()
        .add_event::<UpdateLobby>()
        .add_event::<SendServerMessage>()
        .add_event::<SendClientMessage>();
    }
}