//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2026 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Module for cli argument.

use std::sync::Mutex;

use clap::Parser;

/// Static global variable for enabling debug output.
pub static DEBUG: Mutex<bool> = Mutex::new(false);
pub static LIST: Mutex<bool> = Mutex::new(false);
pub static SHOW: Mutex<u32> = Mutex::new(!0);

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Show verbose output.
  #[arg(short, long)]
  debug: bool,

  /// Show a list with all the entries.
  #[arg(short, long, conflicts_with = "show")]
  list: bool,

  /// Open an specific day.
  #[arg(short, long, conflicts_with = "list")]
  show: Option<u32>,
}

/// Parse and set the program arguments configurations.
pub fn get_arguments() {
  let arguments = Args::parse();
  // Verbose
  if arguments.debug == true {
    let mut locker = DEBUG.lock().expect("Error locking the mutex");
    *locker = true;
  }

  // List
  if arguments.list == true {
    let mut locker = LIST.lock().expect("Error locking the mutex");
    *locker = true;
  }

  // Show
  if let Some(x) = arguments.show {
    let mut locker = SHOW.lock().expect("Error locking the mutex");
    *locker = x;
  }
}
