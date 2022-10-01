use crate::script;

script!(InstallExtraPackagesScript {
    file = "install-extra-packages"
    args = ExtraPackages
});

pub type ExtraPackages = Vec<String>;
