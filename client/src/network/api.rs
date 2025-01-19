use bevy::{math::IVec3, prelude::ResMut};
use bevy_renet::renet::{DefaultChannel, RenetClient};
use bincode::Options;
use shared::messages::{ChatMessageRequest, ClientToServerMessage, SaveWorldRequest};
use shared::world::BlockData;

pub enum NetworkAction {
    ChatMessage(String),
    WorldUpdateRequest {
        requested_chunks: Vec<IVec3>,
        player_chunk_pos: IVec3,
        render_distance: u32,
    },
    SaveWorldRequest,
    BlockInteraction {
        position: IVec3,
        block_type: Option<BlockData>, // None = delete, Some = add
    },
}

pub fn send_network_action(client: &mut ResMut<RenetClient>, action: NetworkAction) {
    match action {
        NetworkAction::ChatMessage(msg) => {
            let input_message = bincode::options()
                .serialize(&ClientToServerMessage::ChatMessage(ChatMessageRequest {
                    content: msg,
                }))
                .unwrap();

            client.send_message(DefaultChannel::ReliableOrdered, input_message);
        }
        NetworkAction::WorldUpdateRequest {
            requested_chunks,
            player_chunk_pos,
            render_distance,
        } => {
            let input_message = bincode::options()
                .serialize(&ClientToServerMessage::WorldUpdateRequest {
                    player_chunk_position: player_chunk_pos,
                    requested_chunks,
                    render_distance,
                })
                .unwrap();

            client.send_message(DefaultChannel::ReliableOrdered, input_message);
        }
        NetworkAction::SaveWorldRequest => {
            let save_request =
                ClientToServerMessage::SaveWorldRequest(SaveWorldRequest { session_token: 0 });

            let input_message = bincode::options()
                .serialize(&save_request)
                .expect("Failed to serialize SaveWorldRequest");

            client.send_message(DefaultChannel::ReliableOrdered, input_message);
        }
        NetworkAction::BlockInteraction {
            position,
            block_type,
        } => {
            let message = bincode::options()
                .serialize(&ClientToServerMessage::BlockInteraction {
                    position,
                    block_type,
                })
                .unwrap();

            client.send_message(DefaultChannel::ReliableOrdered, message);
        }
    }
}
