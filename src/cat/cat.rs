// SPDX-License-Identifier: BSD-3-Clause

use std::{
    fs::{metadata, File},
    io::Read,
    os::unix::fs::MetadataExt,
    path::Path,
};

use anyhow::{anyhow, Context, Ok, Result};
use flate2::bufread::ZlibDecoder;

pub fn cat_file(hash: String, pretty: bool) -> Result<()> {
    let dir_path = format!(
        ".twat/objects/{}{}",
        hash.chars().nth(0).unwrap(),
        hash.chars().nth(1).unwrap(),
    );
    if !Path::new(&dir_path).exists() {
        return Err(anyhow!(
            "[twat]: invalid blob provided, object not in store"
        ));
    }

    let mut file_path_str = String::new();
    for (i, char) in hash.chars().enumerate() {
        if i < 2 {
            continue;
        }
        file_path_str += &char.to_string();
    }
    let file_path = format!("{}/{}", dir_path, file_path_str);
    let path = Path::new(&file_path);
    let mut file =
        File::open(path).context("[twat] unable to open object file")?;
    let md = metadata(&path).unwrap();
    let mut buf: Vec<u8> = vec![0; md.size() as usize];
    file.read(&mut buf).context("[twat]: unable to read file")?;

    let c: &[u8] = &buf;
    let mut d = ZlibDecoder::new(c);
    let mut u: Vec<u8> = Vec::new();
    d.read_to_end(&mut u)
        .context("[twat]: unable to get final compressed bytes")?;

    let idx: Option<usize>;
    let uncompressed = String::from_utf8(u.clone())
        .context("[twat]: unable to convert decompressed blob to string")?;
    idx = uncompressed.find("\\0");
    if idx.is_none() {
        return Err(anyhow!("[twat]: invalid blob file format"));
    }
    let left: &[u8];
    let right: &[u8];
    (left, right) = u.split_at(idx.unwrap() + 2);

    let blob_arr: &[u8];
    let size_arr: &[u8];
    (blob_arr, size_arr) = left.split_at(5);
    let type_str = String::from_utf8(blob_arr.to_vec()).context(
        "[twat]: unable to convert decompressed blob type to string",
    )?;
    let size_str_ended = String::from_utf8(size_arr.to_vec()).context(
        "[twat]: unable to convert decompressed blob content size to string",
    )?;
    if type_str != "blob " {
        return Err(anyhow!("[twat]: unsupported object type"));
    }

    let size_str: &str;
    (size_str, _) = size_str_ended.split_at(size_str_ended.len() - 2);
    let size: usize = size_str
        .parse()
        .context("[twat]: unable to parse file size")?;
    let content = String::from_utf8(right.to_vec())
        .context("[twat]: unable to convert decompressed blob to string")?;
    if size != content.len() {
        return Err(anyhow!(
            "[twat]: blob length doesn't match content length"
        ));
    }

    if pretty {
        println!("{}", content);
        return Ok(());
    }
    println!(
        "{}",
        String::from_utf8(u.clone())
            .context("[twat]: unable to convert decompressed blob to string")?
    );

    return Ok(());
}
