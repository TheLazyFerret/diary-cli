//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Main file of the crate.

use crate::utils::{
  backup_check, check_data_dir, create_backup, create_data, delete_file, get_daily_filename,
  get_data_path, get_editor, restore_backup, run_editor,
};

mod utils;

fn main() -> anyhow::Result<()> {
  // User home data directory.
  let mut main_path = get_data_path()?;
  // Check if the program data directory exist. If not, creates it.
  check_data_dir(&main_path)?;

  // Restore all the pending backup files.
  backup_check(&main_path)?;

  // Push the date.
  main_path.push(get_daily_filename());

  // Data and backup (if needed) paths.
  let backup_path = main_path.with_extension("backup");
  let data_path = main_path.with_extension("txt");

  // env variable from $EDITOR
  let editor = get_editor()?;

  // If the file exist, creates a backup and edit a copy of it.
  if data_path.is_file() {
    create_backup(&data_path, &backup_path)?; // Creates the backup.
    if !run_editor(&editor, &data_path)?.success() {
      // If the editor failed.
      eprintln!("- Editor failed, restoring backup.");
      restore_backup(&data_path, &backup_path)?;
    } else {
      eprintln!("- Editor finished correctly.");
      delete_file(&backup_path)?;
    }
  } else {
    // If the file doesn't exist, creates a new one and is edited directly.
    create_data(&data_path)?;
    if !run_editor(&editor, &data_path)?.success() {
      eprintln!("- Editor failed, deleting posible corrupted data.");
      delete_file(&data_path)?;
    } else {
      eprintln!("- Editor finished correctly.");
    }
  }

  Ok(())
}
