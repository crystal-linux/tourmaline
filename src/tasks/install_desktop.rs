use serde::{Deserialize, Serialize};

use crate::script;

script!(InstallDesktopScript {
    file = "install-desktop"
    args = DesktopConfig
});

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum DesktopConfig {
    Onyx,
    KdePlasma,
    Mate,
    Gnome,
    Cinnamon,
    Xfce,
    Budgie,
    Enlightenment,
    Lxqt,
    Sway,
    I3Gaps,
    HerbstluftWM,
    AwesomeWM,
    BSPWM,
}
