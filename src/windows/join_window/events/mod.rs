use std::net::Ipv4Addr;

use super::*;
use bevy::prelude::*;
pub(super) mod ev_systems;

#[derive(Event)]
pub(super) struct ConnectTo(pub JoinInputs);

#[derive(Event)]
pub struct UpdateHelperText(pub String);