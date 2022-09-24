use serde::Serialize;

use crate::script;

script!(ConfigureLocaleScript {
    file = "configure-locale"
    args = LocaleConfig
});

#[derive(Clone, Serialize, Debug)]
pub struct LocaleConfig {
    pub locale: Vec<String>,
    pub keymap: String,
    pub timezone: String,
}
