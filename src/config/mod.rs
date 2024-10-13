use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::layouts::KeyboardLayoutType;
use serde::Deserialize;

mod constants;

#[derive(Deserialize)]
pub struct Config {
    pub layout: KeyboardLayoutType,
    /// Print a newline between each section
    pub separate_sections: Option<bool>,
    pub indent_size: Option<usize>,
}

impl Config {
    pub fn parse(cwd: &Path, config_file: &Option<PathBuf>) -> Self {
        let rc_file = match config_file {
            Some(file) => file,
            None => &find_rc_file(cwd).expect("Could not find config file"),
        };
        let buf =
            fs::read_to_string(rc_file).expect("Failed to read config file");

        toml::from_str(&buf).expect("Failed to parse config file")
    }

    pub fn with_defaults(layout: KeyboardLayoutType) -> Self {
        Self { layout, indent_size: Some(2), separate_sections: Some(false) }
    }
}

fn find_rc_file(path: &Path) -> Option<PathBuf> {
    let mut path: PathBuf = path.into();
    let file = Path::new(constants::RC_FILENAME);

    // Remove filename if it exists. This happens if the user specifies a path
    // to a single file.
    if path.is_file() {
        path.pop();
    }

    loop {
        path.push(file);

        // If the path exists, we've found it!
        if path.is_file() {
            break Some(path);
        }

        // remove file && remove parent
        if !(path.pop() && path.pop()) {
            break None;
        }
    }
}
