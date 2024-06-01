// SPDX-License-Identifier: BSD-3-Clause

use std::{
    fs::{metadata, set_permissions, DirBuilder, File},
    io::{Read, Write},
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::Path,
};

use anyhow::{Context, Result};
use blake2::{
    digest::{Update, VariableOutput},
    Blake2bVar,
};
use flate2::{write::ZlibEncoder, Compression};
use hex::encode;

pub fn hash_file(path: String, write: bool) -> Result<()> {
    let mut path = Path::new(&path);
    let mut file = File::open(path).context("[twat] unable to open file")?;
    let md = metadata(&path).unwrap();
    let mut buf: Vec<u8> = vec![0; md.size() as usize];
    file.read(&mut buf).context("[twat]: unable to read file")?;

    let mut blob = format!("blob {:?}\\0", md.size()).as_bytes().to_vec();
    blob.extend(&buf);

    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    let u: &[u8] = &blob;
    e.write_all(u)
        .context("[twat]: unable to compress vector")?;
    let compressed = e
        .finish()
        .context("[twat]: unable to get final compressed bytes")?;

    let mut hasher = Blake2bVar::new(20)
        .context("[twat]: unable to create blake2 hasher")?;
    let c: &[u8] = &compressed;
    hasher.update(c);
    let mut hashed_buf = [0u8; 20];
    hasher
        .finalize_variable(&mut hashed_buf)
        .context("[twat]: unable to finalise blake2 hashing")?;

    let hex_blob = encode(hashed_buf);
    if !write {
        println!("{}", hex_blob);
        return Ok(());
    }

    let dir_path = format!(
        ".twat/objects/{}{}",
        hex_blob.chars().nth(0).unwrap(),
        hex_blob.chars().nth(1).unwrap(),
    );
    if !Path::new(&dir_path).exists() {
        path = Path::new(dir_path.as_str());
        DirBuilder::new()
            .recursive(true)
            .create(path)
            .context("[twat]: unable to create blog object directory")?;
        let mut perms = metadata(path)
            .context(format!("unable to get metadata for path {:?}", path))?
            .permissions();
        perms.set_mode(0o755);
        set_permissions(path, perms)
            .context("[twat]: unable to set directory permissions")?;
    }

    let mut file_path_str = String::new();
    for (i, char) in hex_blob.chars().enumerate() {
        if i < 2 {
            continue;
        }
        file_path_str += &char.to_string();
    }
    let file_path = format!("{}/{}", dir_path, file_path_str);
    path = Path::new(&file_path);
    let mut file =
        File::create(path).context("[twat]: unable to open HEAD file")?;
    file.write_all(c)
        .context("[twat]: unable to write to HEAD file")?;
    let mut perms = metadata(path)
        .context(format!("unable to get permissions for path {:?}", path))?
        .permissions();
    perms.set_mode(0o755);
    set_permissions(path, perms)
        .context("[twat]: unable to set directory permissions")?;

    return Ok(());
}
