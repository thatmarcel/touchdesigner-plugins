use std::ffi::CString;
use std::pin::Pin;
use autocxx::prelude::*;
use autocxx::subclass::*;
use td_rs_base::{OperatorInputs, param::{ParameterManager}};

use crate::{Top};

include_cpp! {
    #include "TOP_CPlusPlusBase.h"
    #include "RustTopPlugin.h"
    safety!(unsafe)
    extern_cpp_type!("OP_ParameterManager", td_rs_base::cxx::OP_ParameterManager)
    extern_cpp_type!("OP_String", td_rs_base::cxx::OP_String)
    extern_cpp_type!("OP_InfoDATSize", td_rs_base::cxx::OP_InfoDATSize)
    extern_cpp_type!("OP_InfoCHOPChan", td_rs_base::cxx::OP_InfoCHOPChan)
    extern_cpp_type!("OP_Inputs", td_rs_base::cxx::OP_Inputs)
    generate_pod!("TOP_GeneralInfo")
    generate_pod!("TOP_PluginInfo")

}

pub use td_rs_base::cxx::setString;
pub use ffi::*;

extern "C" {
    fn top_new_impl() -> Box<dyn Top>;
}

#[subclass(superclass("RustTopPlugin"))]
pub struct RustTopPluginImpl {
    inner: Box<dyn Top>,
}

impl Default for RustTopPluginImpl {
    fn default() -> Self {
        unsafe {
            Self {
                inner: top_new_impl(),
                cpp_peer: Default::default(),
            }
        }
    }
}

#[no_mangle]
extern "C" fn top_new() -> *mut RustTopPluginImplCpp {
    RustTopPluginImpl::default_cpp_owned().into_raw()
}

impl RustTopPlugin_methods for RustTopPluginImpl {
    fn getGeneralInfo(&mut self, mut info: Pin<&mut TOP_GeneralInfo>, inputs: &OP_Inputs) {
        // let input = OperatorInputs::new(inputs);
        // let gen_info = self.inner.general_info(&input);
        // info.cookEveryFrame = gen_info.cook_every_frame;
        // info.cookEveryFrameIfAsked = gen_info.cook_every_frame_if_asked;
    }

    fn getOutputFormat(&mut self, mut format: Pin<&mut TOP_OutputFormat>, inputs: &OP_Inputs) -> bool {
        // let input = OperatorInputs::new(inputs);
        // let output_format = self.inner.output_format(&input);
        // format.width = output_format.width;
        // format.height = output_format.height;
        // format.color = output_format.color;
        // format.bitsPerChannel = output_format.bits_per_channel;
        false
    }

    // TOP_OutputFormatSpecs &output_specs, const OP_Inputs &inputs, TOP_Context &context
    fn execute(&mut self, output_specs: Pin<&mut TOP_OutputFormatSpecs>, inputs: &OP_Inputs, context: Pin<&mut TOP_Context>) {
        // let input = OperatorInputs::new(inputs);
        // let mut output = DatOutput::new(outputs);
        // if let Some(mut params) = self.inner.params_mut() {
        //     params.update(&input.params());
        // }
        // self.inner.execute(output, &input);
    }

    fn getNumInfoCHOPChans(&mut self) -> i32 {
        self.inner.num_info_chop_chans() as i32
    }

    fn getInfoCHOPChan(&mut self, index: i32, name: Pin<&mut OP_String>, mut value: Pin<&mut f32>) {
        let (info_name, info_value) = self.inner.info_chop_chan(index as usize);
        unsafe {
            let new_string = CString::new(info_name.as_str()).unwrap();
            let new_string_ptr = new_string.as_ptr();
            name.setString(new_string_ptr);
        }
        value.set(info_value);
    }

    fn getInfoDATSize(&mut self, mut info: Pin<&mut OP_InfoDATSize>) -> bool {
        let (rows, cols) = self.inner.info_dat_size();
        if rows == 0 && cols == 0 {
            false
        } else {
            info.rows = rows as i32;
            info.cols = cols as i32;
            true
        }
    }

    fn getInfoDATEntry(&mut self, index: i32, entryIndex: i32, entry: Pin<&mut OP_String>) {
        let entry_str = self.inner.info_dat_entry(index as usize, entryIndex as usize);
        unsafe {
            let new_string = CString::new(entry_str.as_str()).unwrap();
            let new_string_ptr = new_string.as_ptr();
            entry.setString(new_string_ptr);
        }
    }

    fn getWarningString(&mut self, warning: Pin<&mut OP_String>) {
        unsafe {
            let new_string = CString::new(self.inner.warning()).unwrap();
            let new_string_ptr = new_string.as_ptr();
            warning.setString(new_string_ptr);
        }
    }

    fn getErrorString(&mut self, error: Pin<&mut OP_String>) {
        unsafe {
            let new_string = CString::new(self.inner.error()).unwrap();
            let new_string_ptr = new_string.as_ptr();
            error.setString(new_string_ptr);
        }
    }

    fn getInfoPopupString(&mut self, info: Pin<&mut OP_String>) {
        unsafe {
            let new_string = CString::new(self.inner.info()).unwrap();
            let new_string_ptr = new_string.as_ptr();
            info.setString(new_string_ptr);
        }
    }

    fn setupParameters(&mut self, manager: Pin<&mut OP_ParameterManager>) {
        let params = self.inner.params_mut();
        if let Some(mut params) = params {
            let mut manager = ParameterManager::new(manager);
            params.register(&mut manager);
        }
    }

    unsafe fn pulsePressed(&mut self, name: *const std::ffi::c_char) {
        self.inner.pulse_pressed(std::ffi::CStr::from_ptr(name).to_str().unwrap());
    }
}