use serde::Serialize;

use crate::script;

script!(InstallDesktopScript {
    file = "install-desktop.nu"
    args = DesktopConfig
});

#[derive(Clone, Debug, Serialize)]
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
