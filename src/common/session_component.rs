use bevy::prelude::*;
use bevy_hookup_core::hook_session::SessionId;

#[derive(Component)]
pub struct SessionComponent {
    pub session_id: SessionId,
}
