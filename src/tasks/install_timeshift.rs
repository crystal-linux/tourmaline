use crate::script;

script!(InstallTimeshiftScript {
    file = "install-timeshift"
    args = TimeshiftConfig
});

pub type TimeshiftConfig = ();
