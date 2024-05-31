use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

use crate::misc_error::MiscError;

pub struct UnrealEngineConnectionController {
    socket: UdpSocket
}

impl UnrealEngineConnectionController {
    pub fn new(local_port: u16) -> Result<Self, std::io::Error> {
        let socket_address = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, local_port);

        let socket = match UdpSocket::bind(socket_address) {
            Ok(ds) => ds,
            Err(e) => return Err(e)
        };

        match socket.set_nonblocking(true) {
            Ok(_) => {},
            Err(e) => return Err(e)
        }

        Ok(Self {
            socket
        })
    }

    pub fn receive_value(&self, sender_ip_address: String, sender_port: u16) -> Result<(String, f64), Box<dyn std::error::Error + Send + Sync>> {
        let mut recv_buffer = vec![0u8; u16::MAX as usize];
        let (_received_byte_count, sender_address) = match self.socket.recv_from(&mut recv_buffer) {
            Ok(r) => r,
            Err(e) => return Err(Box::new(e))
        };

        if sender_address.port() == sender_port && sender_address.to_string().split(":").next().unwrap() != sender_ip_address {
            return Err(Box::new(MiscError::NonMatchingMessageSender))
        }

        let message_id = recv_buffer[0];

        if message_id != 0x01 {
            return Err(Box::new(MiscError::UnknownMessageId));
        }

        let value_name_length = recv_buffer[1];

        let value_bytes: [u8; 8] = match recv_buffer[2..2 + 8].try_into() {
            Ok(vb) => vb,
            Err(e) => return Err(Box::new(e))
        };
        let value = f64::from_le_bytes(value_bytes);

        let value_name = match String::from_utf8(recv_buffer[2 + 8..(2 + 8 + value_name_length) as usize].to_vec()) {
            Ok(vn) => vn,
            Err(e) => return Err(Box::new(e))
        };

        Ok((value_name, value))
    }

    pub fn send_value(&self, value_name: String, value: f64, destination_ip_address: String, destination_port: u16) -> Result<usize, std::io::Error> {
        let mut message = [0u8; 2 + 8 + 255];

        let value_name_ascii = value_name.to_ascii_lowercase();
        let mut value_name_bytes = value_name_ascii.as_bytes();

        let value_name_length = value_name_bytes.len();

        message[0] = 0x01;
        message[1] = value_name_length as u8;

        let value_bytes = value.to_le_bytes();

        message[2..2 + 8].copy_from_slice(&value_bytes);

        message[2 + 8..2 + 8 + value_name_length].copy_from_slice(&value_name_bytes);

        self.socket.send_to(&message, format!("{}:{}", destination_ip_address, destination_port))
    }
}