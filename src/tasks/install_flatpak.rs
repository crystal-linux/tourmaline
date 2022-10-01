use crate::script;

script!(InstallFlatpakScript {
    file = "install-flatpak"
    args = FlatpakConfig
});

pub type FlatpakConfig = ();
