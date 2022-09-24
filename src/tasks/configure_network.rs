use serde::Serialize;

use crate::script;

script!(ConfigureNetworkScript {
    file = "configure-network.nu"
    args = NetworkConfig
});

#[derive(Clone, Debug, Serialize)]
pub struct NetworkConfig {
    pub hostname: String,
    pub ipv6_loopback: bool,
}
