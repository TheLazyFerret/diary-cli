//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Util mod for not necessary related functions.

use std::env;
use std::path::PathBuf;

use crate::error::Error;

///$EDITOR env var const.
const EDITOR_VAR: &str = "EDITOR";
/// $XDG_DATA_HOME env var const.
const DATA_HOME_VAR: &str = "XDG_DATA_HOME";
/// $HOME env var const.
const HOME_VAR: &str = "HOME";
/// Default data home when DATA_HOME_VAR is empty.
/// Requires to append at the begin the value of $HOME
const DEFAULT_DATA_HOME: &str = ".local/share";

pub fn fetch_data_home_dir() -> Result<PathBuf, Error> {
  // $HOME is required, should be already checked before calling this function
  assert!(env::var(HOME_VAR).is_ok());
  let result: PathBuf = {
    if let Ok(x) = env::var(DATA_HOME_VAR) {
      PathBuf::from(x)
    } else {
      let mut aux = PathBuf::from(env::var(HOME_VAR).unwrap());
      aux.push(DEFAULT_DATA_HOME);
      aux
    }
  };
  if result.is_dir() {
    Ok(result)
  } else {
    // PathBuf may not be in UTF-8, so may not be able to represent
    if let Some(x) = result.into_os_string().to_str() {
      Err(Error::InvalidPath(x.to_owned()))
    } else {
      Err(Error::InvalidPath("Unable to convert to UTF-8".to_owned()))
    }
  }
} // fn fetch_data_home_dir

#[cfg(test)]
mod test {
  use crate::utils::{DATA_HOME_VAR, fetch_data_home_dir};
  use std::env;

  #[test]
  fn fetch_data_home_test() {
    let test_raw_path = {
      // XDG_DATA_HOME is set (to $HOME)
      if env::var(DATA_HOME_VAR).is_ok() {
        String::from("/home/lazyferret")
      }
      // If it is not set, try fallback ~/.local/share
      else {
        String::from("/home/lazyferret/.local/share")
      }
    };
    let result = fetch_data_home_dir().unwrap();
    assert_eq!(result, test_raw_path);
  }
} // mod test
