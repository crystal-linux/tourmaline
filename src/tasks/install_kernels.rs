use serde::{Deserialize, Serialize};

use crate::script;

script!(InstallKernelsScript {
    file = "install-kernels"
    args = KernelConfig
});

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KernelConfig {
    pub default: Kernel,
    pub additional: Vec<Kernel>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Kernel(pub String);
