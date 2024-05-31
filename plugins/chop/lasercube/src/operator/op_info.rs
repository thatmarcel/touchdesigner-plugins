use td_rs_chop::OpInfo;
use crate::operator::base::LasercubeOp;

impl OpInfo for LasercubeOp {
    const OPERATOR_TYPE: &'static str = "Lasercube";
    const OPERATOR_LABEL: &'static str = "Lasercube";
    const MIN_INPUTS: usize = 1;
    const MAX_INPUTS: usize = 1;
}