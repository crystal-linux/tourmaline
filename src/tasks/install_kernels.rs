use embed_nu::rusty_value::*;
use serde::Deserialize;

use crate::script;

script!(InstallKernelsScript {
    file = "install-kernels"
    args = KernelConfig
});

#[derive(Clone, Debug, RustyValue, Deserialize)]
pub struct KernelConfig {
    pub default: Kernel,
    pub additional: Vec<Kernel>,
}

#[derive(Clone, Debug, RustyValue, Deserialize)]
pub struct Kernel(pub String);
