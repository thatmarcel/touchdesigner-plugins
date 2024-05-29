use packed_struct::{PackedStruct, PackingResult};

use crate::api::lasercube_message::LasercubeMessage;
use crate::api::messages::ids::GET_FULL_INFO_MESSAGE_ID;

pub struct GetFullInfoMessage {}

impl PackedStruct for GetFullInfoMessage {
    type ByteArray = [u8; 0];

    fn pack(&self) -> PackingResult<Self::ByteArray> {
        Ok([0u8; 0])
    }

    fn unpack(_src: &Self::ByteArray) -> PackingResult<Self> {
        Ok(Self {})
    }
}

impl LasercubeMessage for GetFullInfoMessage {
    fn get_message_id() -> u8 {
        GET_FULL_INFO_MESSAGE_ID
    }
}