//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Main file of the crate.

use crate::utils::{
  backup_check, create_backup, create_file, delete_file, get_daily_filename, get_data_path, get_editor, restore_backup, run_editor
};

mod utils;

fn main() -> anyhow::Result<()> {
  let mut main_path = get_data_path()?;
  utils::create_data_dir(&main_path)?;

  backup_check(&main_path)?;
  
  main_path.push(get_daily_filename());

  let backup_path = main_path.with_extension("backup");
  main_path.add_extension("txt");

  let editor = get_editor()?;

  // Creates a backup. If the editor fails, remove the changed and uses the backup (if exists).
  if main_path.is_file() {
    create_backup(&main_path, &backup_path)?;
    if !run_editor(&editor, &main_path)?.success() {
      eprintln!("- Editor failed, restoring backup.");
      restore_backup(&main_path, &backup_path)?;
    } else {
      eprintln!("- Editor finished correctly.");
      delete_file(&backup_path)?;
    }
  } else {
    create_file(&main_path)?;
    if !run_editor(&editor, &main_path)?.success() {
      eprintln!("- Editor failed, deleting posible corrupted data.");
      delete_file(&main_path)?;
    } else {
      eprintln!("- Editor finished correctly.");
    }
  }

  Ok(())
}
