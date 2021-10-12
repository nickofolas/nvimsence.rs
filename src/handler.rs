use crate::config::NvimsenceConfig;
use discord_rich_presence::{
    activity::{self, Assets, Button, Timestamps},
    DiscordIpc,
};
use lazy_static::lazy_static;
use neovim_lib::{Neovim, NeovimApi, Session};
use regex::Regex;
use serde_json::Value;
use std::{
    fs::File,
    io::prelude::*,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

lazy_static! {
    static ref REPO_URL: Regex = Regex::new(r"git@(.*):(.*)$").unwrap();
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const ABBREVIATIONS: [&str; 4] = ["KiB", "MiB", "GiB", "TiB"];

pub struct EventHandler<T: DiscordIpc> {
    pub nvim: Neovim,
    pub rich_presence: T,
    start_time: i64,
    icons: Option<Value>,
    config: Option<NvimsenceConfig>,
}

impl<T: DiscordIpc> EventHandler<T> {
    pub fn new(rich_presence: T) -> Result<Self> {
        let session = Session::new_parent()?;
        let nvim = Neovim::new(session);

        Ok(Self {
            nvim,
            rich_presence,
            start_time: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64,
            icons: None,
            config: None,
        })
    }

    pub fn listen(&mut self) -> Result<()> {
        let receiver = self.nvim.session.start_event_loop_channel();
        self.load()?;
        self.update_presence().unwrap_or(());

        for (event, _) in receiver {
            match &event[..] {
                "update" => {
                    self.update_presence()?;
                }
                "reconnect" => {
                    self.rich_presence.reconnect()?;
                }
                "disconnect" => {
                    self.rich_presence.close()?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn load(&mut self) -> Result<()> {
        let config = NvimsenceConfig::from_nvim(&mut self.nvim)?;
        self.config = Some(config);

        let path = self
            .nvim
            .eval("richPresence#execdir")?
            .as_str()
            .unwrap()
            .to_string()
            + "/icons.json";
        let mut file = File::open(path)?;

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        let icons: Value = serde_json::from_str(&buffer)?;
        self.icons = Some(icons);

        Ok(())
    }

    fn update_presence(&mut self) -> Result<()> {
        let filename = self.get_filename()?;
        let dirname = self.get_project()?;
        let filesize = self.get_filesize()?;
        let line_count = {
            let buf = self.nvim.get_current_buf()?;
            buf.line_count(&mut self.nvim)?
        };

        let details = format!("{}/{}", dirname, filename);
        let state = format!("{} [{} LOC]", filesize, line_count);
        let get_image = self.get_image()?;
        let image = get_image.as_str();

        let mut payload = activity::Activity::new()
            .state(state.as_str())
            .details(details.as_str())
            .timestamps(Timestamps::new().start(self.start_time));

        let filetype = self.get_filetype()?;
        let large_text = format!("Editing a {} file", filetype);
        let assets = Assets::new()
            .large_image(image)
            .large_text(large_text.as_str())
            .small_image("icon-logo")
            .small_text("NeoVim");
        payload = payload.assets(assets);

        let button_url = self.get_button_url()?;
        if !button_url.is_empty() {
            payload = payload.buttons(vec![Button::new("View Repository", &button_url)]);
        }

        self.rich_presence.set_activity(payload).unwrap_or(());

        Ok(())
    }

    fn get_filetype(&mut self) -> Result<String> {
        let value = self.nvim.eval("&filetype")?;
        let filetype = value.as_str().unwrap();

        if !filetype.is_empty() {
            Ok(filetype.to_owned())
        } else {
            let value = self.nvim.eval("expand('%:e')")?;
            let filetype = value.as_str().unwrap();
            Ok(filetype.to_owned())
        }
    }

    fn get_image(&mut self) -> Result<String> {
        let filetype = self.get_filetype()?;

        if self
            .icons
            .as_ref()
            .unwrap()
            .as_object()
            .unwrap()
            .contains_key(&filetype)
        {
            return Ok(self.icons.as_ref().unwrap()[filetype]
                .as_str()
                .unwrap()
                .to_owned());
        } else {
            Ok("text".into())
        }
    }

    fn get_filename(&mut self) -> Result<String> {
        let expanded = self.nvim.eval("expand('%:t')")?;
        let mut filename = expanded.as_str().unwrap_or("");
        if filename.is_empty() {
            filename = "Unknown file";
        }

        Ok(filename.into())
    }

    fn get_directory(&mut self) -> Result<String> {
        let expanded = self.nvim.eval("expand('%:p:h:t')")?;
        let mut dirname = expanded.as_str().unwrap_or("");
        if dirname.is_empty() {
            dirname = "Unknown directory";
        }

        Ok(dirname.into())
    }

    fn get_filesize(&mut self) -> Result<String> {
        let evaluated = self.nvim.eval("wordcount()['bytes']")?;
        let mut bytes = evaluated.as_f64().unwrap_or(1.0);
        let mut suffix = "";

        let base = 1024.0;
        if bytes <= base {
            suffix = " bytes";
        } else {
            for (index, abbrev) in ABBREVIATIONS.iter().enumerate() {
                let value = 2_f64.powi(10).powi((index + 2) as i32);
                if bytes <= value {
                    bytes = (bytes * base) / value;
                    suffix = abbrev;
                    break;
                }
            }
        }

        let filesize = format!("{:.2}{}", bytes, suffix);

        Ok(filesize)
    }

    fn get_project(&mut self) -> Result<String> {
        let path_string = self.nvim.command_output("!git rev-parse --show-toplevel")?;

        if path_string.contains("fatal: not a git repository") {
            self.get_directory()
        } else {
            let path = Path::new(&path_string);

            let path = if let Some(path) = path.file_stem() {
                let mut _path = path.to_string_lossy().into_owned();
                _path.retain(|c| !c.is_whitespace());
                _path
            } else {
                self.get_directory()?
            };

            Ok(path)
        }
    }

    fn get_button_url(&mut self) -> Result<String> {
        let has_fugitive = self
            .nvim
            .eval("exists('g:loaded_fugitive')")?
            .as_i64()
            .unwrap();
        if has_fugitive == 0 {
            return Ok(String::new());
        }

        let mut git_remote = self
            .nvim
            .eval("FugitiveConfigGet('remote.origin.url')")?
            .as_str()
            .unwrap()
            .to_string();
        git_remote.retain(|c| !c.is_whitespace());

        if git_remote.is_empty() {
            return Ok(String::new());
        };

        if REPO_URL.is_match(&git_remote) {
            let captures = REPO_URL.captures(&git_remote).unwrap();
            let (host, path) = (&captures[1], &captures[2]);
            return Ok(format!("https://{}/{}", host, path));
        } else {
            Ok(git_remote)
        }
    }
}
