use crate::windows::{gen_generic_button, gen_generic_description_text};

use super::events::ConnectTo;

use bevy::input::keyboard::{};
use bevy::input::ButtonState;
use bevy::prelude::*;

use super::*;

use super::super::gen_generic_node;
pub(super) mod startup;
pub(super)mod ui_frontend;
pub(super) mod ui_backend;