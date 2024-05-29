use packed_struct::derive::PackedStruct;
use crate::api::lasercube_message::LasercubeMessage;
use crate::api::messages::ids::GET_RING_BUFFER_EMPTY_SAMPLE_COUNT_RESPONSE_MESSAGE_ID;

#[derive(PackedStruct)]
#[packed_struct(bit_numbering="msb0", endian="lsb")]
pub struct GetRingBufferEmptySampleCountResponseMessage {
    #[packed_field(bytes="1:2")]
    pub empty_sample_count: u16
}

impl LasercubeMessage for GetRingBufferEmptySampleCountResponseMessage {
    fn get_message_id() -> u8 {
        GET_RING_BUFFER_EMPTY_SAMPLE_COUNT_RESPONSE_MESSAGE_ID
    }
}