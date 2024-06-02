// SPDX-License-Identifier: BSD-3-Clause

use std::{
    fmt,
    str::{self, FromStr},
};

use anyhow::{anyhow, Error};

#[derive(Debug, PartialEq)]
pub enum ObjectTypesEnum {
    Blob,
}

impl FromStr for ObjectTypesEnum {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "blob" => Ok(ObjectTypesEnum::Blob),
            _ => Err(anyhow!("[twat]: unrecognised object type")),
        }
    }
}

impl fmt::Display for ObjectTypesEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObjectTypesEnum::Blob => write!(f, "blob"),
        }
    }
}
