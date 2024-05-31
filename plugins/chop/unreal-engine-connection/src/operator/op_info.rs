use td_rs_chop::OpInfo;
use crate::operator::base::UnrealEngineConnectionOp;

impl OpInfo for UnrealEngineConnectionOp {
    const OPERATOR_TYPE: &'static str = "Unrealengineconnection";
    const OPERATOR_LABEL: &'static str = "Unreal Engine Connection";
    const MIN_INPUTS: usize = 0;
    const MAX_INPUTS: usize = 1;
    const PYTHON_CALLBACKS_DAT: &'static str = "
def getChannelName(op, channelIndex):
    if len(op.inputs) < 1:
        return \"unnamed\"

    opInput = op.inputs[0]

    channel = opInput.chan(channelIndex)

    return channel.name.encode(\"utf-8\")
";
}