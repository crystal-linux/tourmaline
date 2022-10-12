use embed_nu::rusty_value::*;
use serde::Deserialize;

use crate::script;

script!(ConfigureNetworkScript {
    file = "configure-network"
    args = NetworkConfig
});

#[derive(Clone, Debug, Deserialize, RustyValue)]
pub struct NetworkConfig {
    pub hostname: String,
    pub ipv6_loopback: bool,
}
