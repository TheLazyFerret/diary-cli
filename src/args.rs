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

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Show verbose output.
  #[arg(short, long, action = clap::ArgAction::Count)]
  debug: u8,
}

/// Parse and set the program arguments configurations.
pub fn set_arguments() {
  let arguments = Args::parse();
  // Verbose
  if arguments.debug > 0 {
    let mut locker = DEBUG.lock().expect("Error locking the Mutex");
    *locker = true;
  }
}
