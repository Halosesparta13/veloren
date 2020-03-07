pub mod client;
pub mod ecs_packet;
pub mod server;

// Reexports
pub use self::{
    client::ClientMsg,
    ecs_packet::EcsCompPacket,
    server::{PlayerListUpdate, RegisterError, RequestStateError, ServerInfo, ServerMsg},
};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ClientState {
    Pending,
    Connected,
    Registered,
    Spectator,
    Character,
    Disconnected,
}

pub const MAX_BYTES_CHAT_MSG: usize = 256;

pub enum ChatMsgValidationError {
    TooLong,
}

pub fn validate_chat_msg(msg: &str) -> Result<(), ChatMsgValidationError> {
    // TODO: Consider using grapheme cluster count instead of size in bytes
    if msg.len() <= MAX_BYTES_CHAT_MSG {
        Ok(())
    } else {
        Err(ChatMsgValidationError::TooLong)
    }
}
