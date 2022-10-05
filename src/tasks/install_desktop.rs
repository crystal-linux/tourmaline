use embed_nu::rusty_value::RustyValue;
use serde::Deserialize;

use crate::script;

script!(InstallDesktopScript {
    file = "install-desktop"
    args = DesktopConfig
});

#[derive(Clone, Debug, Deserialize, RustyValue)]
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
