use packed_struct::{PackedStruct, PackingResult};
use packed_struct::prelude::bits::ByteArray;

pub trait LasercubeMessage: PackedStruct {
    fn get_message_id() -> u8;

    fn serialize(&self) -> PackingResult<Vec<u8>> {
        let packed_bytes = match self.pack() {
            Ok(packed) => packed,
            Err(e) => return Err(e)
        };
        
        let packed_bytes_slice = packed_bytes.as_bytes_slice();

        let mut result: Vec<u8> = vec![0u8; packed_bytes_slice.len()];
        
        result.copy_from_slice(packed_bytes_slice);
        
        result.insert(0, Self::get_message_id());

        Ok(result)
    }
}