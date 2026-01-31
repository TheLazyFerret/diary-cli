//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Util mod for not necessary related functions.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Context;

const DEFAULT_XDG_DATA_PATH: &str = ".local/share";

pub const PROGRAM_DIR_DATA_NAME: &str = "diary-cli";

/// Returns the path where the program data will be stored.
pub fn get_data_path() -> anyhow::Result<PathBuf> {
  let mut result: PathBuf;
  if let Ok(x) = env::var("XDG_DATA_HOME") {
    // if XDG_DATA_HOME is set, use that path.
    result = PathBuf::from(x);
    eprintln!("- Data path found: {}", result.to_str().unwrap());

  } else {
    // If not, fallback to default '$HOME/.local/share'
    let home_raw_path = env::var("HOME").context("Failed to retrieve $HOME value")?;
    result = PathBuf::from(home_raw_path);
    result.push(DEFAULT_XDG_DATA_PATH);
    eprintln!("- Data path found: {}", result.to_str().unwrap());
  }
  result.push(PROGRAM_DIR_DATA_NAME);
  eprintln!("- Using the path: {}", result.to_str().unwrap());
  Ok(result)
}

/// Check if the directory exist, if not, attempts to create it.
pub fn create_data_dir(path: &Path) -> anyhow::Result<()> {
  if path.is_dir() {
    eprintln!("- The path already exist: {}", path.to_str().unwrap());
  } else {
    eprintln!("- The path doesn't exist, attempting to create: {}", path.to_str().unwrap());
    fs::create_dir(path).context("Failed to create the data directory")?;
    eprintln!("- The directory was sucesfully created");
  }
  Ok(())
}
