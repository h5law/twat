// SPDX-License-Identifier: BSD-3-Clause

use std::{
    fs::{metadata, set_permissions},
    io::{Read, Write},
    os::unix::fs::PermissionsExt,
};

use anyhow::{Context, Result};
use blake2::{
    digest::{Update, VariableOutput},
    Blake2bVar,
};
use flate2::{bufread::ZlibDecoder, write::ZlibEncoder, Compression};
use hex::encode;

pub fn hash_vector_to_hex(vec: &Vec<u8>) -> Result<String> {
    let mut hasher = Blake2bVar::new(20)
        .context("[twat]: unable to create blake2 hasher")?;
    let c: &[u8] = &vec;
    hasher.update(c);
    let mut hashed_buf = [0u8; 20];
    hasher
        .finalize_variable(&mut hashed_buf)
        .context("[twat]: unable to finalise blake2 hashing")?;

    let hex_str = encode(hashed_buf);
    return Ok(hex_str);
}

pub fn compress_vector(vec: &Vec<u8>) -> Result<Vec<u8>> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    let u: &[u8] = &vec;
    e.write_all(u)
        .context("[twat]: unable to compress vector")?;
    let compressed = e
        .finish()
        .context("[twat]: unable to get final compressed bytes")?;
    return Ok(compressed);
}

pub fn decompress_vector(vec: &Vec<u8>) -> Result<Vec<u8>> {
    let c: &[u8] = &vec;
    let mut d = ZlibDecoder::new(c);
    let mut u: Vec<u8> = Vec::new();
    d.read_to_end(&mut u)
        .context("[twat]: unable to get final decompressed bytes")?;
    return Ok(u);
}

pub fn set_path_755(path: &std::path::Path) -> Result<()> {
    let mut perms = metadata(path)
        .context(format!("unable to get metadata for path {:?}", path))?
        .permissions();
    perms.set_mode(0o755);
    set_permissions(path, perms).context(format!(
        "[twat]: unable to set mode permissions for {:?}",
        path
    ))?;
    return Ok(());
}

pub fn hash_to_object_path(hash: &String) -> String {
    let dir_path = format!(
        ".twat/objects/{}{}",
        hash.chars().nth(0).unwrap(),
        hash.chars().nth(1).unwrap(),
    );
    let mut file_path_str = String::new();
    for (i, char) in hash.chars().enumerate() {
        if i < 2 {
            continue;
        }
        file_path_str += &char.to_string();
    }
    let file_path = format!("{}/{}", dir_path, file_path_str);
    return file_path;
}
