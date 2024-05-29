use packed_struct::derive::PackedStruct;
use crate::api::lasercube_message::LasercubeMessage;
use crate::api::messages::ids::SET_DAC_RATE_MESSAGE_ID;

#[derive(PackedStruct)]
#[packed_struct(bit_numbering="msb0", endian="lsb")]
pub struct SetDacRateMessage {
    #[packed_field(bytes="0:3")]
    pub dac_rate: i32
}

impl LasercubeMessage for SetDacRateMessage {
    fn get_message_id() -> u8 {
        SET_DAC_RATE_MESSAGE_ID
    }
}