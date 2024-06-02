// SPDX-License-Identifier: BSD-3-Clause

use std::{
    fs::{metadata, DirBuilder, File},
    io::Write,
    path::Path,
};

use anyhow::{anyhow, Context, Result};

use crate::utils::utils::set_path_755;

pub fn init_repo() -> Result<()> {
    if Path::new(".twat/").exists() {
        return Err(anyhow!("[twat]: .twat repository already exists"));
    }

    let mut path = Path::new(".twat/objects/refs");
    DirBuilder::new()
        .recursive(true)
        .create(path)
        .context("[twat]: unable to create object references directory")?;
    set_path_755(path)?;
    path = Path::new(".twat/objects/info");
    DirBuilder::new()
        .recursive(true)
        .create(path)
        .context("[twat]: unable to create object info directory")?;
    set_path_755(path)?;
    path = Path::new(".twat/objects/pack");
    DirBuilder::new()
        .recursive(true)
        .create(path)
        .context("[twat]: unable to create object pack directory")?;
    set_path_755(path)?;
    path = Path::new(".twat/refs/heads");
    DirBuilder::new()
        .recursive(true)
        .create(path)
        .context("[twat]: unable to create references head directory")?;
    set_path_755(path)?;
    path = Path::new(".twat/refs/tags");
    DirBuilder::new()
        .recursive(true)
        .create(path)
        .context("[twat]: unable to create references tags directory")?;
    set_path_755(path)?;
    path = Path::new(".twat/");
    assert!(metadata(path).unwrap().is_dir());

    path = Path::new(".twat/HEAD");
    let mut head =
        File::create(path).context("[twat]: unable to open HEAD file")?;
    head.write_all(b"refs/head/main")
        .context("[twat]: unable to write to HEAD file")?;
    set_path_755(path)?;

    Ok(())
}
