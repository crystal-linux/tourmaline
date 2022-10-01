use serde::{Deserialize, Serialize};

use crate::script;

script!(ConfigureNetworkScript {
    file = "configure-network"
    args = NetworkConfig
});

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NetworkConfig {
    pub hostname: String,
    pub ipv6_loopback: bool,
}
