use td_rs_derive::Params;
use td_rs_chop::*;

#[derive(Params, Default, Clone, Debug)]
pub struct UnrealEngineConnectionOpParams {
    #[param(label = "Reconnect")]
    pub reconnect: Pulse,
    #[param(label = "Clear received values")]
    pub clear_received_values: Pulse,
    #[param(label = "Touch Designer Port")]
    pub touch_designer_port: String,
    #[param(label = "Unreal Engine IP address")]
    pub unreal_engine_ip_address: String,
    #[param(label = "Unreal Engine Port")]
    pub unreal_engine_port: String
}