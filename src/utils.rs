//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Util mod for not necessary related functions.

use std::env;
use std::fs;
use std::fs::File;
use std::fs::remove_file;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::Context;
use time::OffsetDateTime;

const DEFAULT_XDG_DATA_PATH: &str = ".local/share";
const PROGRAM_DIR_DATA_NAME: &str = "diary-cli";

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

/// Returns the daily date filename, with extension '.txt'.
pub fn get_daily_filename() -> String {
  let date = OffsetDateTime::now_utc().date().to_string();
  eprintln!("- Retrieved daily date filename: {}", date);
  date
}

/// Creates a backup file of the daily file.
pub fn create_backup(original: &Path, backup: &Path) -> anyhow::Result<()> {
  //debug_assert!(original.is_file() && !backup.is_file());
  File::create(backup).context("Failed to create the backup file")?;
  fs::copy(original, backup).context("Failed to copy the original to the backup")?;
  eprintln!("- Created backup file in: {}", backup.to_str().unwrap());
  Ok(())
}

/// Creates a file in path.
pub fn create_file(path: &Path) -> anyhow::Result<()> {
  debug_assert!(!path.is_file());
  File::create(path).context("Failed to create the daily file")?;
  eprintln!("- Created file in: {}", path.to_str().unwrap());
  Ok(())
}

/// Return the value of $EDITOR.
pub fn get_editor() -> anyhow::Result<String> {
  let value = env::var("EDITOR").context("Failed to retrieve env var $EDITOR")?;
  eprintln!("- Retrieved $EDITOR value: {}", value);
  Ok(value)
}

/// Run the editor child process.
pub fn run_editor(editor: &str, path: &Path) -> anyhow::Result<()> {
  let mut command = Command::new(editor);
  command.arg(path);
  eprintln!("- Command to run: {:?}", command);
  let mut child_handle = command.spawn().context("Failed to spawn editor child process")?;
  child_handle.wait().context("Failed to wait the child")?;
  eprintln!("- Editor finished correctly");
  Ok(())
}

pub fn delete_file(path: &Path) -> anyhow::Result<()> {
  if !path.is_file() {
    eprintln!("- File {} doesn´t exit, skipping delete", path.to_str().unwrap());
    Ok(())
  } else {
    remove_file(path)?;
    eprintln!("- File {} correctly removed", path.to_str().unwrap());
    Ok(())
  }
}