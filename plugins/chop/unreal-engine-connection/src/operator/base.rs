use std::collections::HashMap;
use std::ffi::CStr;
use std::str::FromStr;
use td_rs_chop::{NodeInfo, Op};
use crate::api::unreal_engine_connection_manager::UnrealEngineConnectionManager;
use crate::operator::params::UnrealEngineConnectionOpParams;

pub struct UnrealEngineConnectionOp {
    pub params: UnrealEngineConnectionOpParams,
    pub manager: UnrealEngineConnectionManager,
    pub received_values: HashMap<String, f64>,
    pub output_channel_names: Vec<String>,
    pub input_channel_names: Vec<String>,
    pub execution_number: u8,
    pub node_info: NodeInfo
}

impl UnrealEngineConnectionOp {
    pub fn connect(&mut self) {
        let local_port = match u16::from_str(&*self.params.touch_designer_port) {
            Ok(lp) => lp,
            Err(e) => {
                self.set_warning(format!("Connection failed with error: {}", e).as_str());
                return;
            }
        };

        match self.manager.connect(local_port) {
            Ok(_) => {
                self.set_warning("");
                self.set_error("");
            },
            Err(e) => {
                self.set_warning(format!("Connection failed with error: {}", e).as_str());
            }
        }
    }

    pub fn get_channel_name(&self, channel_index: usize) -> Option<String> {
        let context = self.node_info.context();

        let arguments = context.create_arguments_tuple(1);

        unsafe {
            pyo3_ffi::PyTuple_SET_ITEM(
                arguments,
                1,
                pyo3_ffi::PyLong_FromLong(channel_index as std::ffi::c_long)
            );

            let result = context.call_python_callback("getChannelName", arguments, std::ptr::null_mut());

            if !result.is_null() {
                if pyo3_ffi::PyBytes_Check(result) == 0 {
                    pyo3_ffi::Py_DECREF(result);
                    return None;
                }

                let channel_name = match CStr::from_ptr(pyo3_ffi::PyBytes_AsString(result)).to_str() {
                    Ok(cn) => cn.to_string(),
                    Err(_) => return None
                };

                pyo3_ffi::Py_DECREF(result);

                Some(channel_name)
            } else {
                None
            }
        }
    }
}