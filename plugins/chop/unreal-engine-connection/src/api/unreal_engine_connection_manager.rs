use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

use crate::api::unreal_engine_connection_controller::UnrealEngineConnectionController;
use crate::misc_error::MiscError;

pub struct UnrealEngineConnectionManager {
    controller: Arc<Mutex<Option<UnrealEngineConnectionController>>>,
    kvs: Arc<Mutex<HashMap<String, f64>>>,
    pub is_connected: bool
}

impl UnrealEngineConnectionManager {
    pub fn new() -> Self {
        Self {
            controller: Arc::new(Mutex::new(None)),
            kvs: Arc::new(Mutex::new(HashMap::new())),
            is_connected: false
        }
    }

    pub fn connect(&mut self, local_port: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut controller_mutex_guard = match self.controller.lock() {
            Ok(co) => co,
            Err(_) => return Err(Box::new(MiscError::FailedToLockController))
        };

        *controller_mutex_guard = None;

        let controller = match UnrealEngineConnectionController::new(local_port) {
            Ok(c) => c,
            Err(e) => return Err(Box::new(e))
        };

        *controller_mutex_guard = Some(controller);

        self.is_connected = true;

        Ok(())
    }

    pub fn send_value(&self, value_name: String, value: f64, destination_ip_address: String, destination_port: u16) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let controller_mutex_guard = match self.controller.lock() {
            Ok(co) => co,
            Err(_) => return Err(Box::new(MiscError::FailedToLockController))
        };

        let controller = match controller_mutex_guard.as_ref() {
            Some(c) => c,
            None => return Err(Box::new(MiscError::NotConnected))
        };

        match controller.send_value(value_name, value, destination_ip_address, destination_port) {
            Ok(sbc) => Ok(sbc),
            Err(e) => return Err(Box::new(e))
        }
    }

    pub fn receive_values(&self, sender_ip_address: String, sender_port: u16) -> Result<HashMap<String, f64>, Box<dyn std::error::Error + Send + Sync>> {
        let controller_mutex_guard = match self.controller.lock() {
            Ok(co) => co,
            Err(_) => return Err(Box::new(MiscError::FailedToLockController))
        };

        let controller = match controller_mutex_guard.as_ref() {
            Some(c) => c,
            None => return Err(Box::new(MiscError::NotConnected))
        };

        let mut kvs_mutex_guard = match self.kvs.lock() {
            Ok(k) => k,
            Err(_) => return Err(Box::new(MiscError::FailedToLockController))
        };
        let mut kvs = kvs_mutex_guard.deref_mut();

        loop {
            let (received_value_name, received_value) = match controller.receive_value(sender_ip_address.clone(), sender_port) {
                Ok(rvp) => rvp,
                Err(_) => {
                    // TODO (maybe): Log error

                    break;
                }
            };

            kvs.insert(received_value_name, received_value);
        }

        Ok(kvs.clone())
    }

    pub fn clear_received_values(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut kvs_mutex_guard = match self.kvs.lock() {
            Ok(k) => k,
            Err(_) => return Err(Box::new(MiscError::FailedToLockController))
        };

        let mut kvs = kvs_mutex_guard.deref_mut();

        kvs.drain();

        Ok(())
    }
}