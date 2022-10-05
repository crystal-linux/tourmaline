use embed_nu::rusty_value::RustyValue;
use serde::Deserialize;

use crate::script;

script!(ConfigureLocaleScript {
    file = "configure-locale"
    args = LocaleConfig
});

#[derive(Clone, Deserialize, RustyValue, Debug)]
pub struct LocaleConfig {
    pub locale: Vec<String>,
    pub keymap: String,
    pub timezone: String,
}
