// SPDX-License-Identifier: BSD-3-Clause

use std::{
    fs::{metadata, set_permissions, DirBuilder, File},
    io::Write,
    os::unix::fs::PermissionsExt,
    path::Path,
};

use anyhow::{Context, Result};

pub fn init_repo() -> Result<()> {
    Path::new(".twat/")
        .try_exists()
        .context("[twat] .twat repository already exists")?;

    let mut path = Path::new(".twat/objects/refs");
    DirBuilder::new()
        .recursive(true)
        .create(path)
        .context("[twat]: unable to create object references directory")?;
    let mut perms = metadata(path)?.permissions();
    perms.set_mode(0o755);
    set_permissions(path, perms).context("[twat]: unable to set directory permissions")?;
    path = Path::new(".twat/objects/info");
    DirBuilder::new()
        .recursive(true)
        .create(path)
        .context("[twat]: unable to create object info directory")?;
    perms = metadata(path)?.permissions();
    perms.set_mode(0o755);
    set_permissions(path, perms).context("[twat]: unable to set directory permissions")?;
    path = Path::new(".twat/objects/pack");
    DirBuilder::new()
        .recursive(true)
        .create(path)
        .context("[twat]: unable to create object pack directory")?;
    perms = metadata(path)?.permissions();
    perms.set_mode(0o755);
    set_permissions(path, perms).context("[twat]: unable to set directory permissions")?;
    path = Path::new(".twat/refs/heads");
    DirBuilder::new()
        .recursive(true)
        .create(path)
        .context("[twat]: unable to create references head directory")?;
    perms = metadata(path)?.permissions();
    perms.set_mode(0o755);
    set_permissions(path, perms).context("[twat]: unable to set directory permissions")?;
    path = Path::new(".twat/refs/tags");
    DirBuilder::new()
        .recursive(true)
        .create(path)
        .context("[twat]: unable to create references tags directory")?;
    perms = metadata(path)?.permissions();
    perms.set_mode(0o755);
    set_permissions(path, perms).context("[twat]: unable to set directory permissions")?;
    path = Path::new(".twat/");
    assert!(metadata(path).unwrap().is_dir());

    path = Path::new(".twat/HEAD");
    let mut head = File::create(path).context("[twat]: unable to open HEAD file")?;
    head.write_all(b"refs/head/main")
        .context("[twat]: unable to write to HEAD file")?;
    perms = metadata(path)?.permissions();
    perms.set_mode(0o755);
    set_permissions(path, perms).context("[twat]: unable to set directory permissions")?;

    return Ok(());
}
