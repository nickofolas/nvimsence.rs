use neovim_lib::{Neovim, NeovimApi};
use std::collections::HashMap;

pub struct NvimsenceConfig {
    keys: HashMap<String, String>,
}

impl NvimsenceConfig {
    pub fn from_nvim(nvim: &mut Neovim) -> Result<Self, Box<dyn std::error::Error>> {
        let mut keys: HashMap<String, String> = HashMap::new();

        for key in ["g:nvimsence_details", "g:nvimsence_nothing"] {
            if let Ok(val) = nvim.eval(key) {
                if keys.insert(key.to_string(), val.to_string()).is_none() {
                    continue;
                };
            };
        }
        // nvim.command(&format!("echo '{:?}'", keys))?;

        Ok(NvimsenceConfig { keys })
    }

    pub fn get(&self, key: &str) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(val) = self.keys.get(key) {
            return Ok(val.to_owned());
        };
        Ok(String::from(""))
    }
}
