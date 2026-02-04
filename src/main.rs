//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2026 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Main file of the crate.

use std::path::PathBuf;

use crate::utils::{
  backup_check, check_data_dir, get_daily_filename, get_data_path, get_date_from_index, get_editor,
  list_data_files, open_file,
};

use crate::args::get_arguments;

mod args;
mod utils;

fn main() -> anyhow::Result<()> {
  // Parse and set the program args params.
  get_arguments();

  // User home data directory.
  let data_path = get_data_path()?;

  // Check if the program data directory exist. If not, creates it.
  check_data_dir(&data_path)?;

  // Restore all the pending backup files.
  backup_check(&data_path)?;

  if *args::LIST.lock().expect("Error locking mutex") == true {
    return list_data_files(&data_path);
  }

  let data_index = *args::SHOW.lock().expect("Error locking mutex");

  // Check if the index is valid

  let paths: (PathBuf, PathBuf);
  if data_index != !0 {
    if let Some(aux) = get_date_from_index(&data_path, data_index)? {
      let data_path = aux.with_extension("txt");
      let backup_path = aux.with_added_extension("backup");
      paths = (data_path, backup_path);
    } else {
      println!("The index {} is not valid", data_index);
      return Ok(());
    }
  } else {
    let mut aux = data_path.clone();
    aux.push(get_daily_filename());
    let data_path = aux.with_extension("txt");
    let backup_path = aux.with_extension("backup");
    paths = (data_path, backup_path);
  }

  // env variable from $EDITOR
  let editor = get_editor()?;

  // Open the selected file.
  open_file(&paths.0, &paths.1, &editor)
}
