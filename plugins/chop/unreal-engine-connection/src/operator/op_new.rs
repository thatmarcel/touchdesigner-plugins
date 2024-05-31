use std::collections::HashMap;
use td_rs_chop::{NodeInfo, OpNew};
use crate::api::unreal_engine_connection_manager::UnrealEngineConnectionManager;
use crate::operator::base::{UnrealEngineConnectionOp};
use crate::operator::params::{UnrealEngineConnectionOpParams};

impl OpNew for UnrealEngineConnectionOp {
    fn new(node_info: NodeInfo) -> Self {
        UnrealEngineConnectionOp {
            params: UnrealEngineConnectionOpParams {
                reconnect: Default::default(),
                clear_received_values: Default::default(),
                touch_designer_port: "39451".to_string(),
                unreal_engine_ip_address: "127.0.0.1".to_string(),
                unreal_engine_port: "39452".to_string(),
            },
            manager: UnrealEngineConnectionManager::new(),
            received_values: HashMap::new(),
            output_channel_names: Vec::new(),
            input_channel_names: Vec::new(),
            execution_number: 0u8,
            node_info
        }
    }
}