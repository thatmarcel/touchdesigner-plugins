use std::sync::Arc;
use std::sync::atomic::{AtomicU16, AtomicU8, Ordering};

use crate::api::lasercube_connection::LasercubeConnection;
use crate::api::lasercube_point::LasercubePoint;
use crate::api::messages::get_full_info_message::GetFullInfoMessage;
use crate::api::messages::get_full_info_response_message::GetFullInfoResponseMessage;
use crate::api::messages::get_ring_buffer_empty_sample_count_message::GetRingBufferEmptySampleCountMessage;
use crate::api::messages::get_ring_buffer_empty_sample_count_response_message::GetRingBufferEmptySampleCountResponseMessage;
use crate::api::messages::ids::FRAME_SAMPLES_MESSAGE_ID;
use crate::api::messages::set_buffer_size_response_enabled_message::SetBufferSizeResponseEnabled;
use crate::api::messages::set_dac_rate_message::SetDacRateMessage;
use crate::api::messages::set_output_enabled_message::SetOutputEnabledMessage;
use crate::misc_error::MiscError;

pub struct LasercubeController {
    pub connection: LasercubeConnection,
    pub ring_buffer_empty_sample_count: Arc<AtomicU16>,
    pub frame_number: Arc<AtomicU8>
}

impl LasercubeController {
    pub fn new(lasercube_ip_address: String) -> Result<Self, std::io::Error> {
        let connection = match LasercubeConnection::new(lasercube_ip_address) {
            Ok(lc) => lc,
            Err(e) => return Err(e)
        };

        Ok(Self {
            connection,
            ring_buffer_empty_sample_count: Arc::new(AtomicU16::new(0)),
            frame_number: Arc::new(AtomicU8::new(0))
        })
    }

    pub fn set_buffer_size_response_enabled(&self, is_enabled: bool) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let message = SetBufferSizeResponseEnabled {
            is_enabled: is_enabled as u8
        };

        self.connection.send_command(message)
    }

    pub fn set_output_enabled(&self, is_enabled: bool) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let message = SetOutputEnabledMessage {
            is_enabled: is_enabled as u8
        };

        self.connection.send_command(message)
    }

    pub fn set_dac_rate(&self, dac_rate: u32) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let message = SetDacRateMessage {
            dac_rate: dac_rate as i32
        };

        self.connection.send_command(message)
    }

    pub fn request_full_info(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let message = GetFullInfoMessage {};

        match self.connection.send_command(message) {
            Ok(_) => {},
            Err(e) => return Err(e)
        };

        Ok(())
    }

    pub fn request_ring_buffer_empty_sample_count(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let message = GetRingBufferEmptySampleCountMessage {};

        match self.connection.send_command(message) {
            Ok(_) => {},
            Err(e) => return Err(e)
        };

        Ok(())
    }

    pub fn handle_next_command_message(&self) -> Result<(), Box<dyn std::error::Error>> {
        let message_id = match self.connection.recv_command_message_id() {
            Ok(mi) => mi,
            Err(e) => return Err(e)
        };

        match message_id {
            crate::api::messages::ids::GET_FULL_INFO_RESPONSE_MESSAGE_ID => {
                let message = match self.connection.recv_command::<GetFullInfoResponseMessage>() {
                    Ok(m) => m,
                    Err(e) => return Err(e)
                };
                
                self.ring_buffer_empty_sample_count.store(message.ring_buffer_empty_sample_count, Ordering::SeqCst);
            },
            crate::api::messages::ids::GET_RING_BUFFER_EMPTY_SAMPLE_COUNT_RESPONSE_MESSAGE_ID => {
                let message = match self.connection.recv_command::<GetRingBufferEmptySampleCountResponseMessage>() {
                    Ok(m) => m,
                    Err(e) => return Err(e)
                };

                self.ring_buffer_empty_sample_count.store(message.empty_sample_count, Ordering::SeqCst);
            },
            _ => {
                self.connection.recv_and_disregard_all_commands();
            }
        }

        Ok(())
    }

    fn send_frame_samples_message(&self, message_number: u8, points: &[LasercubePoint]) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let mut points_data = vec![0u8; 0];

        for point in points {
            points_data.extend_from_slice(&point.x.min(4095).to_le_bytes());
            points_data.extend_from_slice(&point.y.min(4095).to_le_bytes());
            points_data.extend_from_slice(&point.r.min(4095).to_le_bytes());
            points_data.extend_from_slice(&point.g.min(4095).to_le_bytes());
            points_data.extend_from_slice(&point.b.min(4095).to_le_bytes());
        }
        
        let mut message_data = vec![
            FRAME_SAMPLES_MESSAGE_ID,
            0u8,
            message_number,
            self.frame_number.load(Ordering::SeqCst)
        ];
        
        message_data.append(&mut points_data);

        let sent_bytes_count = match self.connection.data_socket.send(&message_data) {
            Ok(sbc) => sbc,
            Err(e) => return Err(Box::new(e))
        };
            
        Ok(sent_bytes_count)
    }

    fn send_frame_samples_message_if_enough_space(&self, message_number: u8, points: &[LasercubePoint]) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        if self.ring_buffer_empty_sample_count.load(Ordering::SeqCst) >= 2000 {
            self.send_frame_samples_message(message_number, points)
        } else {
            Err(Box::new(MiscError::NoSpaceInBuffer))
        }
    }

    pub fn send_frame_samples(&self, points: Vec<LasercubePoint>) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let frame_number = self.frame_number.fetch_add(1u8, Ordering::SeqCst);

       if frame_number % 2 == 0 {
            match self.request_ring_buffer_empty_sample_count() {
                Ok(_) => {},
                Err(e) => return Err(e)
            }
        }
        
        let mut sent_byte_count = 0usize;

        for (chunk_index, points_chunk) in points.chunks(140).enumerate() {
            match self.send_frame_samples_message_if_enough_space(chunk_index as u8, points_chunk) {
                Ok(sbc) => {
                    sent_byte_count += sbc;
                    
                    _ = self.ring_buffer_empty_sample_count.fetch_update(
                        Ordering::SeqCst,
                        Ordering::SeqCst,
                        |cv| cv.checked_sub(points_chunk.len() as u16)
                    );
                },
                Err(e) => return Err(e)
            };
        }

        Ok(sent_byte_count)
    }
}