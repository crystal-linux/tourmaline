use crate::script;

script!(InstallZRamDScript {
    file = "install-zramd"
    args = ZRamDConfig
});

pub type ZRamDConfig = ();
