use super::*;
pub(super) mod ev_systems;

#[derive(Event)]
pub(super) struct TryConnectionTo(pub JoinInputs);

#[derive(Event)]
pub struct UpdateHelperText(pub String);