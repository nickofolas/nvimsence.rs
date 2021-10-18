use neovim_lib::{Neovim, NeovimApi};
use std::collections::HashMap;

pub struct NvimsenceConfig {
    keys: HashMap<String, String>,
}

impl NvimsenceConfig {
    pub fn from_nvim(nvim: &mut Neovim) -> Result<Self, Box<dyn std::error::Error>> {
        let mut keys: HashMap<String, String> = HashMap::new();

        for key in ["g:nvimsence_details", "g:nvimsence_state"] {
            if let Ok(val) = nvim.eval(key) {
                if let Some(val) = val.as_str() {
                    keys.insert(key.to_string(), val.to_string());
                }
            };
        }

        for key in ["g:nvimsence_show_elapsed", "g:nvimsence_show_buttons"] {
            if let Ok(val) = nvim.eval(key) {
                keys.insert(
                    key.to_string(),
                    match val.as_i64() {
                        Some(0) => "false".into(),
                        _ => "true".into(),
                    },
                );
            }
        }

        Ok(NvimsenceConfig { keys })
    }

    pub fn get(&self, key: &str) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(val) = self.keys.get(key) {
            return Ok(val.clone());
        };
        Ok(String::from(""))
    }
}
