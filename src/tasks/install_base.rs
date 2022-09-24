use crate::script;

script!(InstallBaseScript {
    file = "install-base.nu"
    args = InstallBaseArgs
});

type InstallBaseArgs = ();
