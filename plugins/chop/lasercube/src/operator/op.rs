use td_rs_chop::{Op, OperatorParams};
use crate::operator::base::LasercubeOp;

impl Op for LasercubeOp {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }

    fn pulse_pressed(&mut self, name: &str) {
        if name == "Reconnect" {
            self.connect();
        }
    }
}