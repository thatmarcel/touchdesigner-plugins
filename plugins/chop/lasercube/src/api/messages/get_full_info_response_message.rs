use packed_struct::derive::PackedStruct;
use crate::api::lasercube_message::LasercubeMessage;
use crate::api::messages::ids::GET_FULL_INFO_RESPONSE_MESSAGE_ID;

#[derive(PackedStruct)]
#[packed_struct(bit_numbering="msb0", endian="lsb", size_bytes="63")]
pub struct GetFullInfoResponseMessage {
    #[packed_field(bytes="2")]
    pub firmware_version_major: u8,

    #[packed_field(bytes="3")]
    pub firmware_version_minor: u8,

    #[packed_field(bytes="4")]
    pub flags: u8, // TODO (maybe): Split flags into separate variables

    #[packed_field(bytes="9:12")]
    pub dac_rate: i32,

    #[packed_field(bytes="13:16")]
    pub max_dac_rate: i32,

    #[packed_field(bytes="18:19")]
    pub ring_buffer_empty_sample_count: u16,

    #[packed_field(bytes="20:21")]
    pub ring_buffer_size: u16,

    #[packed_field(bytes="22")]
    pub battery_percentage: u8,

    #[packed_field(bytes="23")]
    pub temperature: u8,

    #[packed_field(bytes="24")]
    pub connection_type: u8,

    #[packed_field(bytes="25:30")]
    pub serial_number_bytes: [u8; 6],

    #[packed_field(bytes="31:34")]
    pub ip_address_bytes: [u8; 4],

    #[packed_field(bytes="36")]
    pub model_number: u8
}

impl LasercubeMessage for GetFullInfoResponseMessage {
    fn get_message_id() -> u8 {
        GET_FULL_INFO_RESPONSE_MESSAGE_ID
    }
}