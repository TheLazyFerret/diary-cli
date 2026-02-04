//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2026 TheLazyFerret
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

const UTF8_UNWRAP_ERROR: &str = "Error converting path to UTF-8";

/// Returns the path where the program data will be stored.
pub fn get_data_path() -> anyhow::Result<PathBuf> {
  let mut result: PathBuf;
  if let Ok(x) = env::var("XDG_DATA_HOME") {
    // if XDG_DATA_HOME is set, use that path.
    result = PathBuf::from(x);
    print_debug(&format!(
      "Home data directory found: {}",
      result.to_str().expect(UTF8_UNWRAP_ERROR)
    ));
  } else {
    // If not, fallback to default '$HOME/.local/share'
    let home_raw_path = env::var("HOME").context("Failed to retrieve $HOME value")?;
    result = PathBuf::from(home_raw_path);
    result.push(DEFAULT_XDG_DATA_PATH);
    print_debug(&format!(
      "Home data directory found: {}",
      result.to_str().expect(UTF8_UNWRAP_ERROR)
    ));
  }
  result.push(PROGRAM_DIR_DATA_NAME);
  print_debug(&format!("Program data directory: {}", result.to_str().expect(UTF8_UNWRAP_ERROR)));
  Ok(result)
}

/// Check if the data directory exist, if not, attempts to create it.
pub fn check_data_dir(path: &Path) -> anyhow::Result<()> {
  if path.is_dir() {
    print_debug(&format!(
      "The data directory already exist: {}",
      path.to_str().expect(UTF8_UNWRAP_ERROR)
    ));
  } else {
    print_debug(&format!(
      "The data directory doesn't exist, will be created in: {}",
      path.to_str().expect("Error converting to UTF-8")
    ));
    fs::create_dir(path).context("Failed to create the data directory")?;
    print_debug("The data directory was sucesfully created");
  }
  Ok(())
}

/// Returns the daily date filename, with extension '.txt'.
pub fn get_daily_filename() -> String {
  let date = OffsetDateTime::now_utc().date().to_string();
  print_debug(&format!("Retrieved date: {}", date));
  date
}

/// Creates a backup file of the daily file.
pub fn create_backup(original: &Path, backup: &Path) -> anyhow::Result<()> {
  debug_assert!(original.is_file() && !backup.is_file());
  File::create(backup).context("Failed to create the backup file")?;
  fs::copy(original, backup).context("Failed to copy the original to the backup")?;
  print_debug(&format!("Created backup file in: {}", backup.to_str().expect(UTF8_UNWRAP_ERROR)));
  Ok(())
}

/// Creates a file in path.
pub fn create_data(path: &Path) -> anyhow::Result<()> {
  debug_assert!(!path.is_file());
  File::create(path).context("Failed to create the data file")?;
  print_debug(&format!("Created data file in: {}", path.to_str().expect(UTF8_UNWRAP_ERROR)));
  Ok(())
}

/// Return the value of $EDITOR.
pub fn get_editor() -> anyhow::Result<String> {
  let value = env::var("EDITOR").context("Failed to retrieve env var $EDITOR")?;
  print_debug(&format!("Retrieved $EDITOR value: {}", value));
  Ok(value)
}

/// Run the editor child process.
pub fn run_editor(editor: &str, path: &Path) -> anyhow::Result<ExitStatus> {
  let mut command = Command::new(editor);
  command.arg(path);
  print_debug(&format!("Command to run: {:?}", command));
  let mut child_handle = command.spawn().context("Failed to spawn editor child process")?;
  Ok(child_handle.wait().context("Failed to wait the child")?)
}

/// Function wrapper for removing a file from the filesystem.
pub fn delete_file(path: &Path) -> anyhow::Result<()> {
  if !path.is_file() {
    print_debug(&format!(
      "File {} doesn´t exit, skipping delete",
      path.to_str().expect(UTF8_UNWRAP_ERROR)
    ));
    Ok(())
  } else {
    fs::remove_file(path).context("Failed to remove the file")?;
    print_debug(&format!("File {} correctly removed", path.to_str().expect(UTF8_UNWRAP_ERROR)));
    Ok(())
  }
}

/// Copies the content from the backup to the main file.
pub fn restore_backup(main: &Path, backup: &Path) -> anyhow::Result<()> {
  debug_assert!(main.is_file() && backup.is_file());
  fs::copy(&backup, &main).context("Failed to copy the data in the backup")?;
  print_debug("Backup correctly restored");
  delete_file(&backup)?;
  Ok(())
}

/// Restore all the '.backup' file in the data directory.
pub fn backup_check(path: &Path) -> anyhow::Result<()> {
  // Retrieve the .backup file path list.
  let backup_files: Vec<PathBuf> = get_files_with_extension(&path, "backup")?;
  if backup_files.is_empty() {
    print_debug("No backup to restore");
    return Ok(());
  }
  print_debug(&format!("Found {} backups to restore", backup_files.len()));
  for backup_file in backup_files {
    debug_assert!(backup_file.exists());
    let mut data_file = backup_file.clone();
    data_file.set_extension("txt");
    if !data_file.is_file() {
      create_data(&data_file)?;
    } // Creates the file if not exist
    restore_backup(&data_file, &backup_file)?; // Restore the content.
    print_debug(&format!(
      "Correctly restored backup: {}",
      backup_file.to_str().expect(UTF8_UNWRAP_ERROR)
    ));
  }
  eprintln!();
  print_debug("Finish backup restauration");
  Ok(())
}

/// Returns a vector with all files in the directory path.
pub fn get_files_in_directory(path: &Path) -> anyhow::Result<Vec<PathBuf>> {
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
  Ok(vec)
}

/// Return a vector with all the files with 'filter_extension' as extension.
pub fn get_files_with_extension(path: &Path, extension: &str) -> anyhow::Result<Vec<PathBuf>> {
  let files_vec = get_files_in_directory(path)?;
  let mut filtered_vec: Vec<PathBuf> = Vec::new();
  // Second loop for getting the .backup files.
  for entry in files_vec {
    if let Some(file_ext) = entry.extension() {
      if file_ext.to_str().expect("Error unwrapping OsStr extension") == extension {
        filtered_vec.push(entry);
      }
    }
  }
  Ok(filtered_vec)
}

/// Check and print verbose logs in error output if enabled.
pub fn print_debug(message: &str) {
  let value = crate::args::DEBUG.lock().expect("Error locking mutex");
  if *value == true {
    eprintln!("[DEBUG] {}", message);
  }
}

/// Open a file with the selected editor. If already exist, then first creates a backup.
pub fn open_file(data: &Path, backup: &Path, editor: &str) -> anyhow::Result<()> {
  // If the file exist, creates a backup and edit a copy of it.
  if data.is_file() {
    create_backup(&data, &backup)?; // Creates the backup.
    if !run_editor(&editor, &data)?.success() {
      // If the editor failed.
      println!("- Editor failed, restoring backup.");
      restore_backup(&data, &backup)?;
    } else {
      delete_file(&backup)?;
    }
  } else {
    // If the file doesn't exist, creates a new one and is edited directly.
    create_data(&data)?;
    if !run_editor(&editor, &data)?.success() {
      println!("- Editor failed, deleting posible corrupted data.");
      delete_file(&data)?;
    }
  }
  Ok(())
}

/// List the data files in data path, and show them in standard output, sorted.
pub fn list_data_files(data_path: &Path) -> anyhow::Result<()> {
  print_debug(&format!("listing files in: {}", data_path.to_str().expect(UTF8_UNWRAP_ERROR)));
  let mut vec = get_files_with_extension(data_path, "txt")?;
  if vec.len() == 0 {
    println!("No files found");
    return Ok(());
  }
  vec.sort();
  for file in vec.iter().enumerate() {
    let aux_path = file.1.to_owned().with_extension("");
    let filename = aux_path.file_name().unwrap().to_str().expect(UTF8_UNWRAP_ERROR);
    println!("[{}] : {}", file.0, filename);
  }
  Ok(())
}

/// Return a Option<PathBuf> from the data file list in data_path, sorted.
pub fn get_date_from_index(data_path: &Path, index: usize) -> anyhow::Result<Option<PathBuf>> {
  let str_path = data_path.to_str().expect(UTF8_UNWRAP_ERROR);
  print_debug(&format!("Retrieving [{}] from: {}", index, str_path));
  let mut vec = get_files_with_extension(data_path, "txt")?;
  vec.sort();
  if let Some(x) = vec.get(index) {
    print_debug(&format!("Found: {}", x.to_str().expect(UTF8_UNWRAP_ERROR)));
    Ok(Some(x.with_extension("")))
  } else {
    print_debug("No file found");
    Ok(None)
  }
}
