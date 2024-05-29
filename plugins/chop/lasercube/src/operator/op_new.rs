use td_rs_chop::{NodeInfo, OpNew};
use crate::api::lasercube_manager::LasercubeManager;
use crate::operator::base::LasercubeOp;
use crate::operator::params::LasercubeOpParams;

impl OpNew for LasercubeOp {
    fn new(_info: NodeInfo) -> Self {
        LasercubeOp {
            params: LasercubeOpParams {
                reconnect: Default::default(),
                lasercube_ip_address: "192.168.1.1".to_string(),
                scale: 1.0,
                brightness: 1.0,
                sample_rate: 30000
            },
            manager: LasercubeManager::new(),
            execution_number: 0u8
        }
    }
}