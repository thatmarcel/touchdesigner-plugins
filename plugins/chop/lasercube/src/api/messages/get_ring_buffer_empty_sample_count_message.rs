use packed_struct::{PackedStruct, PackingResult};

use crate::api::lasercube_message::LasercubeMessage;
use crate::api::messages::ids::GET_RING_BUFFER_EMPTY_SAMPLE_COUNT_MESSAGE_ID;

pub struct GetRingBufferEmptySampleCountMessage {}

impl PackedStruct for GetRingBufferEmptySampleCountMessage {
    type ByteArray = [u8; 0];

    fn pack(&self) -> PackingResult<Self::ByteArray> {
        Ok([0u8; 0])
    }

    fn unpack(_src: &Self::ByteArray) -> PackingResult<Self> {
        Ok(Self {})
    }
}

impl LasercubeMessage for GetRingBufferEmptySampleCountMessage {
    fn get_message_id() -> u8 {
        GET_RING_BUFFER_EMPTY_SAMPLE_COUNT_MESSAGE_ID
    }
}