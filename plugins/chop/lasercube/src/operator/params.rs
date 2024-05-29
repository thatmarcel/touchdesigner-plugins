use td_rs_derive::Params;
use td_rs_chop::*;

#[derive(Params, Default, Clone, Debug)]
pub struct LasercubeOpParams {
    #[param(label = "Reconnect")]
    pub reconnect: Pulse,
    #[param(label = "IP address")]
    pub lasercube_ip_address: String,
    #[param(label = "Scale")]
    pub scale: f64,
    #[param(label = "Brightness")]
    pub brightness: f64,
    #[param(label = "Sample rate")]
    pub sample_rate: u32
}