use packed_struct::derive::PackedStruct;
use crate::api::lasercube_message::LasercubeMessage;
use crate::api::messages::ids::SET_OUTPUT_ENABLED_MESSAGE_ID;

#[derive(PackedStruct)]
#[packed_struct(bit_numbering="msb0", endian="lsb")]
pub struct SetOutputEnabledMessage {
    #[packed_field(bytes="0", size_bytes="1")]
    pub is_enabled: u8
}

impl LasercubeMessage for SetOutputEnabledMessage {
    fn get_message_id() -> u8 {
        SET_OUTPUT_ENABLED_MESSAGE_ID
    }
}