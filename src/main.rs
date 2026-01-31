//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Main file of the crate.

mod utils;

fn main() -> anyhow::Result<()> {
  let data_path = utils::get_data_path()?;
  utils::create_data_dir(&data_path)?;
  Ok(())
}
