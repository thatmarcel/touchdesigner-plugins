use td_rs_chop::{Chop, ChopGeneralInfo, ChopInput, ChopOutput, Op, OperatorInputs};

use crate::api::lasercube_point::LasercubePoint;
use crate::operator::base::LasercubeOp;

impl Chop for LasercubeOp {
    fn general_info(&self, _input: &OperatorInputs<ChopInput>) -> ChopGeneralInfo {
        ChopGeneralInfo {
            cook_every_frame: true,
            cook_every_frame_if_asked: true,
            timeslice: true,
            input_match_index: 0
        }
    }

    fn execute(&mut self, _output: &mut ChopOutput, inputs: &OperatorInputs<ChopInput>) {
        self.execution_number += 1;

        if self.execution_number % 100 == 0 && !self.manager.is_connected {
            self.connect();
        }

        let first_input = match inputs.input(0) {
            Some(fi) => fi,
            None => {
                self.set_warning("No samples connected to input");
                return;
            }
        };
        
        if first_input.num_samples() < 1 {
            self.set_warning("No samples found");
            return;
        }

        if first_input.num_channels() < 5 {
            self.set_warning("Not enough channels found");
            return;
        }
        
        let mut frame_samples: Vec<LasercubePoint> = vec![LasercubePoint::default(); first_input.num_samples()];

        for sample_index in 0..first_input.num_samples() {
            let mut point = LasercubePoint::default();

            for channel_index in 0..first_input.num_channels() {
                let cell_content_number = first_input.channel(channel_index)[sample_index];

                match channel_index {
                    0 => { point.x = (cell_content_number * (self.params.scale as f32) * 2047.5f32 + 2047.5f32) as u16; },
                    1 => { point.y = (cell_content_number * (self.params.scale as f32) * 2047.5f32 + 2047.5f32) as u16; },
                    2 => { point.r = (cell_content_number * (self.params.brightness as f32) * 4095f32) as u16; },
                    3 => { point.g = (cell_content_number * (self.params.brightness as f32) * 4095f32) as u16; },
                    4 => { point.b = (cell_content_number * (self.params.brightness as f32) * 4095f32) as u16; },
                    _ => {}
                };
            }

            frame_samples[sample_index] = point;
        }

        let _sent_bytes_count = self.manager.send_frame_samples(frame_samples);

        /* let frame_samples_count = frame_samples.len();
        
        let _sent_bytes_count = match self.manager.send_frame_samples(frame_samples) {
            Ok(sbc) => sbc,
            Err(e) => {
                self.set_warning(format!("Sending samples failed with error: {} (frame samples count: {})", e, frame_samples_count).as_str());
                return;
            }
        }; */

        // self.set_warning(format!("Ring buffer empty sample count: {:?}", self.manager.get_ring_buffer_empty_sample_count()).as_str());
        self.set_warning("");
        self.set_error("");
    }
}