use std::str::FromStr;
use td_rs_chop::{Chop, ChopGeneralInfo, ChopInput, ChopOutput, ChopOutputInfo, Op, OperatorInputs};
use crate::operator::base::UnrealEngineConnectionOp;

impl Chop for UnrealEngineConnectionOp {
    fn channel_name(&self, index: usize, _input: &OperatorInputs<ChopInput>) -> String {
        if self.output_channel_names.len() > index {
            self.output_channel_names[index].clone()
        } else {
            "unnamed".to_string()
        }
    }

    fn execute(&mut self, output: &mut ChopOutput, input: &OperatorInputs<ChopInput>) {
        self.execution_number += 1;

        if self.execution_number % 100 == 0 && !self.manager.is_connected {
            self.connect();
        }

        let unreal_engine_port = match u16::from_str(&*self.params.unreal_engine_port) {
            Ok(lp) => lp,
            Err(e) => {
                self.set_warning(format!("Connection failed with error: {}", e).as_str());
                return;
            }
        };

        match input.input(0) {
            Some(first_input) => {
                let is_reloading_channel_names = self.execution_number % 100 == 0;

                if is_reloading_channel_names {
                    self.input_channel_names = vec![String::new(); first_input.num_channels()];
                }

                for channel_index in 0..first_input.num_channels() {
                    let channel_values = first_input.channel(channel_index);

                    if channel_values.len() < 1 {
                        continue;
                    }

                    let channel_value = channel_values[0];

                    let channel_name = match is_reloading_channel_names {
                        true => match self.get_channel_name(channel_index) {
                            Some(cn) => {
                                self.input_channel_names[channel_index] = cn.clone();

                                cn.clone()
                            },
                            None => continue
                        },
                        false => {
                            if self.input_channel_names.len() <= channel_index {
                                continue;
                            }

                            self.input_channel_names[channel_index].clone()
                        }
                    };

                    if channel_name.is_empty() {
                        continue;
                    }

                    _ = self.manager.send_value(
                        channel_name,
                        channel_value as f64,
                        self.params.unreal_engine_ip_address.clone(),
                        unreal_engine_port
                    );
                }
            },
            None => {}
        };

        self.received_values = match self.manager.receive_values(self.params.unreal_engine_ip_address.clone(), unreal_engine_port) {
            Ok(rv) => rv,
            Err(e) => {
                self.set_warning(format!("Receiving values failed with error: {}", e).as_str());
                return;
            }
        };

        self.output_channel_names = vec![String::new(); self.received_values.len()];

        for (index, (key, value)) in self.received_values.iter().enumerate() {
            if output.num_channels() <= index {
                break;
            }

            output[index][0] = *value as f32;
            self.output_channel_names[index] = key.clone();
        }
    }

    fn general_info(&self, _input: &OperatorInputs<ChopInput>) -> ChopGeneralInfo {
        ChopGeneralInfo {
            cook_every_frame: true,
            cook_every_frame_if_asked: true,
            timeslice: false,
            input_match_index: 0
        }
    }

    fn output_info(&self, _input: &OperatorInputs<ChopInput>) -> Option<ChopOutputInfo> {
        Some(ChopOutputInfo {
            num_channels: self.received_values.len() as u32,
            num_samples: 1,
            start_index: 0,
            ..Default::default()
        })
    }
}