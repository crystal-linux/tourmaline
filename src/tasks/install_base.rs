use crate::script;

script!(InstallBaseScript {
    file = "install-base"
    args = InstallBaseArgs
});

type InstallBaseArgs = ();
