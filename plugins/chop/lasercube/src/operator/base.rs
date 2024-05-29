use td_rs_chop::Op;
use crate::api::lasercube_manager::LasercubeManager;
use crate::operator::params::LasercubeOpParams;

pub struct LasercubeOp {
    pub params: LasercubeOpParams,
    pub manager: LasercubeManager,
    pub execution_number: u8
}

impl LasercubeOp {
    pub fn connect(&mut self) {
        match self.manager.connect(self.params.lasercube_ip_address.clone(), self.params.sample_rate) {
            Ok(_) => {
                self.set_warning("");
                self.set_error("");
            },
            Err(e) => {
                self.set_warning(format!("Connection failed with error: {}", e).as_str());
            }
        }
    }
}