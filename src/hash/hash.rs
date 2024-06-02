// SPDX-License-Identifier: BSD-3-Clause

use std::{
    fs::{metadata, set_permissions, DirBuilder, File},
    io::{Read, Write},
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::Path,
};

use anyhow::{anyhow, Context, Result};

use crate::utils::utils::{
    compress_vector, hash_to_object_path, hash_vector_to_hex, set_path_755,
};

pub fn hash_file(path: String, write: bool, print: bool) -> Result<()> {
    if !Path::new(".twat/").exists() {
        return Err(anyhow!("[twat]: .twat repository doesn't exists"));
    }

    let mut path = Path::new(&path);
    let mut file = File::open(path).context("[twat] unable to open file")?;
    let md = metadata(&path).unwrap();
    let mut buf: Vec<u8> = vec![0; md.size() as usize];
    file.read(&mut buf).context("[twat]: unable to read file")?;

    let mut blob = format!("blob {:?}\0", md.size()).as_bytes().to_vec();
    blob.extend(&buf);
    let compressed = compress_vector(&blob)?;
    let hex_blob = hash_vector_to_hex(&compressed)?;

    if !write {
        println!("{}", hex_blob);
        return Ok(());
    }

    let file_path = hash_to_object_path(&hex_blob);
    if !Path::new(&file_path)
        .parent()
        .context("[twat]: error getting parent directory for file")?
        .exists()
    {
        path = Path::new(&file_path)
            .parent()
            .context("[twat]: error getting parent directory for file")?;
        DirBuilder::new()
            .recursive(true)
            .create(path)
            .context("[twat]: unable to create blog object directory")?;
        set_path_755(&path)?;
    }

    path = Path::new(&file_path);
    let mut file =
        File::create(path).context("[twat]: unable to open HEAD file")?;
    file.write_all(&compressed)
        .context("[twat]: unable to write to blob object file")?;
    let mut perms = metadata(path)
        .context(format!("unable to get permissions for path {:?}", path))?
        .permissions();
    perms.set_mode(0o755);
    set_permissions(path, perms)
        .context("[twat]: unable to set directory permissions")?;

    if print {
        println!("{}", hex_blob);
    }

    return Ok(());
}
