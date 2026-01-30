//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Util mod for not necessary related functions.

use std::env;
use std::path::PathBuf;

use crate::error::Error;

/// All this env variable must be set before running the program
const REQUIRED_ENV_VARIABLES: [&str; 2] = ["EDITOR", "HOME"];
/// Default data home when DATA_HOME_VAR is empty.
/// Requires to append at the begin the value of $HOME
const DEFAULT_DATA_HOME: &str = ".local/share";

pub fn fetch_data_home_dir() -> Result<PathBuf, Error> {
  // $HOME is required, should be already checked before calling this function
  assert!(env::var("HOME").is_ok());
  let result: PathBuf = {
    if let Ok(x) = env::var("XDG_DATA_HOME") {
      PathBuf::from(x)
    } else {
      let mut aux = PathBuf::from(env::var("HOME").unwrap());
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

pub fn check_required_env() -> Option<Vec<Error>> {
  let vec: Vec<Error> = REQUIRED_ENV_VARIABLES
    .iter()
    .filter(|x| env::var(x).is_err())
    .map(|x| Error::RequiredEnvVarNotFound(x.to_string()))
    .collect();
  if vec.len() != 0 { Some(vec) } else { None }
}

#[cfg(test)]
mod test {
  use crate::utils::{fetch_data_home_dir, check_required_env};
  use std::env;

  #[test]
  fn fetch_data_home_test() {
    let test_raw_path = {
      // XDG_DATA_HOME is set (to $HOME)
      if env::var("XDG_DATA_HOME").is_ok() {
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
  
  #[test]
  fn check_required_env_test() {
    // Will always fail if all the env vars in REQUIRED_ENV_VAR are not set
    assert!(check_required_env().is_none());
  }
} // mod test
