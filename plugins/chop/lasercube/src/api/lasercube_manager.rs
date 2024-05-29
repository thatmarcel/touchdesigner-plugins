use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering;
use std::thread::sleep;
use std::time::Duration;

use crate::api::lasercube_controller::LasercubeController;
use crate::api::lasercube_point::LasercubePoint;
use crate::misc_error::MiscError;

pub struct LasercubeManager {
    controller: Arc<Mutex<Option<LasercubeController>>>,
    pub is_connected: bool
}

impl LasercubeManager {
    pub fn new() -> Self {
        let mut result = Self {
            controller: Arc::new(Mutex::new(None)),
            is_connected: false
        };

        result.start_recv_loop();

        result
    }

    pub fn connect(&mut self, lasercube_ip_address: String, sample_rate: u32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut controller_mutex_guard = match self.controller.lock() {
            Ok(cmg) => cmg,
            Err(_) => return Err(Box::new(MiscError::FailedToLockController))
        };

        *controller_mutex_guard = None;
        
        self.is_connected = false;

        let controller = match LasercubeController::new(
            lasercube_ip_address
        ) {
            Ok(lc) => lc,
            Err(e) => return Err(Box::new(e))
        };

        match controller.set_buffer_size_response_enabled(false) {
            Ok(_) => {},
            Err(e) => return Err(e)
        };

        match controller.set_output_enabled(true) {
            Ok(_) => {},
            Err(e) => return Err(e)
        };

        match controller.set_dac_rate(sample_rate) {
            Ok(_) => {},
            Err(e) => return Err(e)
        };

        match controller.request_full_info() {
            Ok(_) => {},
            Err(e) => return Err(e)
        };

        *controller_mutex_guard = Some(controller);
        
        self.is_connected = true;

        Ok(())
    }

    pub fn start_recv_loop(&mut self) {
        let controller_arc = Arc::clone(&self.controller);

        std::thread::spawn(move || {
            loop {
                sleep(Duration::from_millis(1));

                let controller_mutex_guard = match controller_arc.lock() {
                    Ok(cmg) => cmg,
                    Err(_) => continue
                };

                let controller = match controller_mutex_guard.as_ref() {
                    Some(c) => c,
                    None => continue
                };

                match controller.handle_next_command_message() {
                    Ok(_) => {},
                    Err(_e) => {
                        // TODO: Log error
                    }
                }
            }
        });
    }

    pub fn send_frame_samples(
        &self,
        points: Vec<LasercubePoint>
    ) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let controller_mutex_guard = match self.controller.lock() {
            Ok(cmg) => cmg,
            Err(_) => return Err(Box::new(MiscError::FailedToLockController))
        };

        let controller = match controller_mutex_guard.as_ref() {
            Some(c) => c,
            None => return Err(Box::new(MiscError::NotConnected))
        };

        controller.send_frame_samples(points)
    }

    #[allow(dead_code)]
    pub fn is_lasercube_ip_address_different(&self, lasercube_ip_address: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let controller_mutex_guard = match self.controller.lock() {
            Ok(cmg) => cmg,
            Err(_) => return Err(Box::new(MiscError::FailedToLockController))
        };

        let controller = match controller_mutex_guard.as_ref() {
            Some(c) => c,
            None => return Err(Box::new(MiscError::NotConnected))
        };

        Ok(controller.connection.lasercube_ip_address != lasercube_ip_address)
    }

    #[allow(dead_code)]
    pub fn get_ring_buffer_empty_sample_count(&self) -> Result<u16, Box<dyn std::error::Error + Send + Sync>> {
        let controller_mutex_guard = match self.controller.lock() {
            Ok(cmg) => cmg,
            Err(_) => return Err(Box::new(MiscError::FailedToLockController))
        };

        let controller = match controller_mutex_guard.as_ref() {
            Some(c) => c,
            None => return Err(Box::new(MiscError::NotConnected))
        };

        Ok(controller.ring_buffer_empty_sample_count.load(Ordering::SeqCst))
    }

    #[allow(dead_code)]
    pub fn get_frame_number(&self) -> Result<u8, Box<dyn std::error::Error + Send + Sync>> {
        let controller_mutex_guard = match self.controller.lock() {
            Ok(cmg) => cmg,
            Err(_) => return Err(Box::new(MiscError::FailedToLockController))
        };

        let controller = match controller_mutex_guard.as_ref() {
            Some(c) => c,
            None => return Err(Box::new(MiscError::NotConnected))
        };

        Ok(controller.frame_number.load(Ordering::SeqCst))
    }
}