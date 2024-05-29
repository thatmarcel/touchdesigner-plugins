use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::time::Duration;

use packed_struct::{PackedStruct, PackedStructSlice};
use packed_struct::prelude::bits::ByteArray;

use crate::api::lasercube_message::LasercubeMessage;
use crate::misc_error::MiscError;

const PORT_COMMAND: u16 = 45457;
const PORT_DATA: u16 = 45458;

const READ_TIMEOUT_DURATION: Duration = Duration::from_millis(5);

pub struct LasercubeConnection {
    pub lasercube_ip_address: String,
    pub data_socket: UdpSocket,
    command_socket: UdpSocket
}

impl LasercubeConnection {
    pub fn new(lasercube_ip_address: String) -> Result<Self, std::io::Error> {
        let data_socket = match Self::bind_new_udp_socket() {
            Ok(ds) => ds,
            Err(e) => return Err(e)
        };

        match data_socket.connect(format!("{}:{}", lasercube_ip_address, PORT_DATA)) {
            Ok(_) => {},
            Err(e) => return Err(e)
        }

        _ = data_socket.set_read_timeout(Some(READ_TIMEOUT_DURATION));

        let command_socket = match Self::bind_new_udp_socket() {
            Ok(ds) => ds,
            Err(e) => return Err(e)
        };

        match command_socket.connect(format!("{}:{}", lasercube_ip_address, PORT_COMMAND)) {
            Ok(_) => {},
            Err(e) => return Err(e)
        }

        _ = command_socket.set_read_timeout(Some(READ_TIMEOUT_DURATION));

        Ok(Self {
            lasercube_ip_address,
            data_socket,
            command_socket
        })
    }

    fn bind_new_udp_socket() -> Result<UdpSocket, std::io::Error> {
        let socket_address = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
        UdpSocket::bind(socket_address)
    }

    pub fn recv_data<T: LasercubeMessage>(&self) -> Result<T, Box<dyn std::error::Error + Send + Sync>> {
        Self::recv(&self.data_socket)
    }

    pub fn recv_command<T: LasercubeMessage>(&self) -> Result<T, Box<dyn std::error::Error + Send + Sync>> {
        Self::recv(&self.command_socket)
    }

    pub fn recv_data_message_id(&self) -> Result<u8, Box<dyn std::error::Error + Send + Sync>> {
        Self::recv_message_id(&self.data_socket)
    }

    pub fn recv_command_message_id(&self) -> Result<u8, Box<dyn std::error::Error + Send + Sync>> {
        Self::recv_message_id(&self.command_socket)
    }

    pub fn send_data<T: LasercubeMessage>(&self, message_data: T) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        Self::send(&self.data_socket, message_data)
    }

    pub fn send_command<T: LasercubeMessage>(&self, message_data: T) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        Self::send(&self.command_socket, message_data)
    }

    pub fn recv_and_disregard_all_data(&self) {
        Self::recv_and_disregard_all(&self.command_socket);
    }

    pub fn recv_and_disregard_all_commands(&self) {
        Self::recv_and_disregard_all(&self.command_socket);
    }

    pub fn recv_and_disregard_all(socket: &UdpSocket) {
        let mut recv_buffer = [0u8; 4096];
        _ = socket.recv(&mut recv_buffer);
    }

    fn recv<T: LasercubeMessage>(socket: &UdpSocket) -> Result<T, Box<dyn std::error::Error + Send + Sync>> {
        let mut recv_buffer = vec![0u8; T::ByteArray::len() + 1];
        let _received_byte_count = match socket.recv(&mut recv_buffer) {
            Ok(rbc) => rbc,
            Err(e) => return Err(Box::new(e))
        };

        // Starting at the second bit to ignore message id
        return match T::unpack_from_slice(&recv_buffer[1..]) {
            Ok(result) => Ok(result),
            Err(e) => Err(Box::new(e))
        };
    }

    fn recv_message_id(socket: &UdpSocket) -> Result<u8, Box<dyn std::error::Error + Send + Sync>> {
        let mut recv_buffer: [u8; 1] = [0u8; 1];
        let _received_byte_count = match socket.peek(&mut recv_buffer) {
            Ok(rbc) => rbc,
            Err(e) => return Err(Box::new(e))
        };

        if recv_buffer.len() > 0 {
            Ok(recv_buffer[0])
        } else {
            Err(Box::new(MiscError::EmptyBufferReceived))
        }
    }

    fn send<T: LasercubeMessage>(socket: &UdpSocket, message_data: T) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let message_bytes = match message_data.serialize() {
            Ok(packed) => packed,
            Err(e) => return Err(Box::new(e))
        };

        match socket.send(&message_bytes.as_slice()) {
            Ok(sbc) => Ok(sbc),
            Err(e) => return Err(Box::new(e))
        }
    }
}