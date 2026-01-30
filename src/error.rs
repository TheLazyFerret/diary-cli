//! Author: TheLazyFerret <https://github.com/TheLazyFerret>
//! Copyright (c) 2025 TheLazyFerret
//!   Licensed under the MIT license.
//!   See LICENSE file in the project root for full license information.
//!
//! Crate error module.

use thiserror::Error;

#[derive(Clone, Error, Debug)]
pub enum Error {
  #[error("Required environment variable '{0}' was not found.")]
  RequiredEnvVarNotFound(String),
  #[error("Invalid path '{0}' was found.")]
  InvalidPath(String),
}
