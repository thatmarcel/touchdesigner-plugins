#![feature(concat_bytes)]

use td_rs_chop::*;
use crate::operator::base::LasercubeOp;

mod operator;
mod api;
mod misc_error;

chop_plugin!(LasercubeOp);