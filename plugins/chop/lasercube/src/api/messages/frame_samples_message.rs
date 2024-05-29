use packed_struct::derive::PackedStruct;

use crate::api::lasercube_message::LasercubeMessage;
use crate::api::messages::ids::FRAME_SAMPLES_MESSAGE_ID;

#[derive(PackedStruct)]
#[packed_struct(bit_numbering="msb0", endian="lsb")]
pub struct FrameSamplesMessage {
    #[packed_field(bytes="0:0")]
    pub status_code: u8,

    #[packed_field(bytes="1:1")]
    pub message_number: u8,

    #[packed_field(bytes="2:2")]
    pub frame_number: u8,

    #[packed_field(bytes="3:1402")]
    pub point_data: [u16; 700]
}

impl LasercubeMessage for FrameSamplesMessage {
    fn get_message_id() -> u8 {
        FRAME_SAMPLES_MESSAGE_ID
    }
}