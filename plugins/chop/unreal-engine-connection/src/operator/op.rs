use td_rs_chop::{Op, OperatorParams};
use crate::operator::base::UnrealEngineConnectionOp;

impl Op for UnrealEngineConnectionOp {
    fn params_mut(&mut self) -> Option<Box<&mut dyn OperatorParams>> {
        Some(Box::new(&mut self.params))
    }

    fn pulse_pressed(&mut self, name: &str) {
        if name == "Reconnect" {
            self.connect();
        } else if name.starts_with("Clear") {
            match self.manager.clear_received_values() {
                Ok(_) => {},
                Err(e) => self.set_warning(format!("Connection failed with error: {}", e).as_str())
            };
        }
    }
}