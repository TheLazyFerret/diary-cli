//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Util mod for not necessary related functions.

use std::env;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::process::ExitStatus;

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
    eprintln!("- Home data directory found: {}", result.to_str().unwrap());
  } else {
    // If not, fallback to default '$HOME/.local/share'
    let home_raw_path = env::var("HOME").context("Failed to retrieve $HOME value")?;
    result = PathBuf::from(home_raw_path);
    result.push(DEFAULT_XDG_DATA_PATH);
    eprintln!("- Home data directory found: {}", result.to_str().unwrap());
  }
  result.push(PROGRAM_DIR_DATA_NAME);
  eprintln!("- Program data directory: {}", result.to_str().unwrap());
  Ok(result)
}

/// Check if the data directory exist, if not, attempts to create it.
pub fn check_data_dir(path: &Path) -> anyhow::Result<()> {
  if path.is_dir() {
    eprintln!("- The data directory already exist: {}", path.to_str().unwrap());
  } else {
    eprintln!("- The data directory doesn't exist, will be created in: {}", path.to_str().unwrap());
    fs::create_dir(path).context("Failed to create the data directory")?;
    eprintln!("- The data directory was sucesfully created");
  }
  Ok(())
}

/// Returns the daily date filename, with extension '.txt'.
pub fn get_daily_filename() -> String {
  let date = OffsetDateTime::now_utc().date().to_string();
  eprintln!("- Retrieved date: {}", date);
  date
}

/// Creates a backup file of the daily file.
pub fn create_backup(original: &Path, backup: &Path) -> anyhow::Result<()> {
  debug_assert!(original.is_file() && !backup.is_file());
  File::create(backup).context("Failed to create the backup file")?;
  fs::copy(original, backup).context("Failed to copy the original to the backup")?;
  eprintln!("- Created backup file in: {}", backup.to_str().unwrap());
  Ok(())
}

/// Creates a file in path.
pub fn create_data(path: &Path) -> anyhow::Result<()> {
  debug_assert!(!path.is_file());
  File::create(path).context("Failed to create the data file")?;
  eprintln!("- Created data file in: {}", path.to_str().unwrap());
  Ok(())
}

/// Return the value of $EDITOR.
pub fn get_editor() -> anyhow::Result<String> {
  let value = env::var("EDITOR").context("Failed to retrieve env var $EDITOR")?;
  eprintln!("- Retrieved $EDITOR value: {}", value);
  Ok(value)
}

/// Run the editor child process.
pub fn run_editor(editor: &str, path: &Path) -> anyhow::Result<ExitStatus> {
  let mut command = Command::new(editor);
  command.arg(path);
  eprintln!("- Command to run: {:?}", command);
  let mut child_handle = command.spawn().context("Failed to spawn editor child process")?;
  Ok(child_handle.wait().context("Failed to wait the child")?)
}

/// Function wrapper for removing a file from the filesystem.
pub fn delete_file(path: &Path) -> anyhow::Result<()> {
  if !path.is_file() {
    eprintln!("- File {} doesn´t exit, skipping delete", path.to_str().unwrap());
    Ok(())
  } else {
    fs::remove_file(path).context("Failed to remove the file")?;
    eprintln!("- File {} correctly removed", path.to_str().unwrap());
    Ok(())
  }
}

/// Copies the content from the backup to the main file.
pub fn restore_backup(main: &Path, backup: &Path) -> anyhow::Result<()> {
  debug_assert!(main.is_file() && backup.is_file());
  fs::copy(&backup, &main).context("Failed to copy the data in the backup")?;
  eprintln!("- Backup correctly restored");
  delete_file(&backup)?;
  Ok(())
}

/// Restore all the '.backup' file in the data directory.
pub fn backup_check(path: &Path) -> anyhow::Result<()> {
  // Retrieve the .backup file path list.
  let backup_files: Vec<PathBuf> = get_backup_files(path)?;
  if backup_files.is_empty() {
    eprintln!("- No backup to restore");
    return Ok(());
  }
  eprintln!("- Found {} backups to restore", backup_files.len());
  for backup_file in backup_files {
    debug_assert!(backup_file.exists());
    let mut data_file = backup_file.clone();
    data_file.set_extension("txt");
    if !data_file.is_file() {
      create_data(&data_file)?;
    } // Creates the file if not exist
    restore_backup(&data_file, &backup_file)?; // Restore the content.
    eprintln!("- Correctly restored backup: {}", backup_file.to_str().unwrap());
  }
  eprintln!("- Finished backup restore");
  Ok(())
}

/// Returns a vector with all the Path of the files with extension '.backup' in the directory.
pub fn get_backup_files(path: &Path) -> anyhow::Result<Vec<PathBuf>> {
  let mut vec: Vec<PathBuf> = Vec::new();
  let entry_list = path.read_dir().context("Error reading directory")?;
  // First loop for getting files in the directory.
  for entry in entry_list {
    if let Ok(unwrap_entry) = entry {
      let entry_path = unwrap_entry.path();
      if entry_path.is_file() {
        vec.push(entry_path);
      }
    }
  }
  let mut curated_vec: Vec<PathBuf> = Vec::new();
  // Second loop for getting the .backup files.
  for entry in vec {
    if let Some(extension) = entry.extension() {
      if extension.to_str().expect("Error unwrapping OsStr extension") == "backup" {
        curated_vec.push(entry);
      }
    }
  }
  Ok(curated_vec)
}
