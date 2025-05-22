mod keybindings;
mod styles;

use std::path::PathBuf;

use color_eyre::eyre::Result;
use config::ConfigError;
use serde::Deserialize;

use serde_json;

use crate::utils;

const CONFIG: &str = include_str!("../.config/config.json5");

const DEFAULT_CONFIG: &str = r#"{
  "keybindings": {
    "Home": {

      "<up>": "ScrollUp",            // Scroll up the page
      "<k>": "ScrollUp",             // Scroll up the page
      "<Shift-k>": "ScrollUp",             // Scroll up the page

      "<down>": "ScrollDown",        // Scroll down the page
      "<j>": "ScrollDown",           // Scroll down the page
      "<Shift-j>": "ScrollDown",           // Scroll down the page

      "<home>": "ScrollToTop",       // Scroll to top of the page
      "<Shift-h>": "ScrollToTop",          // Scroll to top of the page

      "<end>": "ScrollToBottom",     // Scroll to bottom of the page
      "<Shift-l>": "ScrollToBottom",       // Scroll to bottom of the page

      "<a>": "React", // Like a post // React to the post
      "<l>": "React", // Like a post // React to the post
      "<r>": "Repost",               // Repost the post
      "<u>": "Repost",               // Repost the post

      "<esc>": "Unselect",           // Unselect the posts

      "<q>": "Quit",                 // Quit the application

      "<Ctrl-d>": "Quit",            // Another way to quit
      "<Ctrl-c>": "Quit",            // Yet another way to quit

      "<Ctrl-z>": "Suspend",         // Suspend the application

      "<n>": "NewTextNote",          // Show the text note input form

      "<c>": "NewTextNote",          // Show the text note input form

      "<Shift-r>": "ReplyTextNote",  // Show the text note input form to reply

      "<Ctrl-space>": "SubmitTextNote",  // Submit the text note on input form
      "<Ctrl-p>": "SubmitTextNote",  // Submit the text note on input form
      "<Ctrl-q>": "SubmitTextNote"   // Submit the text note on input form
    }
  },
  "relays": [
    "wss://nos.lol",
    "wss://relay.damus.io",
    "wss://yabu.me",
    "wss://relay-jp.nostr.wirednet.jp"
  ]
}"#;

const DEFAULT_USER_CONFIG: &str = r#"{"privatekey": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855","relays": ["wss://relay.damus.io", "wss://e.nos.lol"]}"#;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub _data_dir: PathBuf,
    #[serde(default)]
    pub _config_dir: PathBuf,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default, flatten)]
    pub config: AppConfig,
    #[serde(default)]
    pub keybindings: keybindings::KeyBindings,
    #[serde(default)]
    pub styles: styles::Styles,
    #[serde(default)]
    pub privatekey: String,
    #[serde(default)]
    pub relays: Vec<String>,
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        let default_config: Config = json5::from_str(DEFAULT_CONFIG).unwrap();
        let data_dir = utils::get_data_dir();
        let config_dir = utils::get_config_dir();
        let mut builder = config::Config::builder()
            .set_default("_data_dir", data_dir.to_str().unwrap())?
            .set_default("_config_dir", config_dir.to_str().unwrap())?;

        let config_files = [
            //("config.json5", config::FileFormat::Json5),
            ("config.json", config::FileFormat::Json),
            //("config.yaml", config::FileFormat::Yaml),
            //("config.toml", config::FileFormat::Toml),
            //("config.ini", config::FileFormat::Ini),
        ];
        let mut found_config = false;
        for (file, format) in &config_files {
            builder = builder.add_source(
                config::File::from(config_dir.join(file))
                    .format(*format)
                    .required(false),
            );
            if config_dir.join(file).exists() {
                found_config = true
            } else {
                let file_path = PathBuf::from(config_dir.clone()).join(file);
                std::fs::write(&file_path, DEFAULT_USER_CONFIG);
            }
        }
        if !found_config {
            log::info!("No configuration file found");
        }

        let mut cfg: Self = builder.build()?.try_deserialize()?;

        for (mode, default_bindings) in default_config.keybindings.iter() {
            let user_bindings = cfg.keybindings.entry(*mode).or_default();
            for (key, cmd) in default_bindings.iter() {
                user_bindings
                    .entry(key.clone())
                    .or_insert_with(|| cmd.clone());
            }
        }
        for (mode, default_styles) in default_config.styles.iter() {
            let user_styles = cfg.styles.entry(*mode).or_default();
            for (style_key, style) in default_styles.iter() {
                user_styles
                    .entry(style_key.clone())
                    .or_insert_with(|| *style);
            }
        }

        if cfg.privatekey.is_empty() {
            return Err(ConfigError::NotFound(String::from("privatekey")));
        }

        if cfg.relays.is_empty() {
            cfg.relays = default_config.relays.clone();
        }

        Ok(cfg)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_config() {
        assert_eq!(Config::new().is_err(), true);

        // let c = Config::new()?;
        // assert_eq!(
        //     c.keybindings
        //         .get(&Mode::Home)
        //         .unwrap()
        //         .get(&parse_key_sequence("<q>").unwrap_or_default())
        //         .unwrap(),
        //     &Action::Quit
        // );
        // Ok(())
    }
}
