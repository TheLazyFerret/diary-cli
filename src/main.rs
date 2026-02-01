//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Main file of the crate.

use crate::utils::{
  create_backup, create_file, get_daily_filename, get_data_path, get_editor, remove_file, run_editor
};

mod utils;

fn main() -> anyhow::Result<()> {
  let mut main_path = get_data_path()?;
  utils::create_data_dir(&main_path)?;
  main_path.push(get_daily_filename());

  let backup_path = main_path.with_extension("backup");
  main_path.add_extension("txt");

  let editor = get_editor()?;

  // Creates a backup. If the editor fails, remove the changed and uses the backup.
  if main_path.is_file() {
    create_backup(&main_path, &backup_path)?;
    run_editor(&editor, &main_path)?;
    remove_file(&backup_path)?;
  } else {
    create_file(&main_path)?;
    run_editor(&editor, &main_path)?;
  }

  Ok(())
}
