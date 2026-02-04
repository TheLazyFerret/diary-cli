//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2026 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Main file of the crate.

use crate::utils::{
  backup_check, check_data_dir, get_daily_filename, get_data_path, get_editor, open_file,
};

use crate::args::set_arguments;

mod args;
mod utils;

fn main() -> anyhow::Result<()> {
  // Parse and set the program args params.
  set_arguments();

  if *args::LIST.lock().expect("Error locking mutex") == true {
    
  }

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

  // Open the selected file.
  open_file(&data_path, &backup_path, &editor)
}
