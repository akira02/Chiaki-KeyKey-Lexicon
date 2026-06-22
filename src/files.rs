use crate::types::FileInfo;
use anyhow::{bail, Context, Result};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};

pub fn verify_required_files(paths: &[PathBuf]) -> Result<()> {
    let missing = paths
        .iter()
        .filter(|path| !path.is_file())
        .collect::<Vec<_>>();
    if missing.is_empty() {
        return Ok(());
    }

    eprintln!("Missing required file(s):");
    for path in missing {
        eprintln!("  {}", path.display());
    }
    eprintln!();
    eprintln!("Run `cargo run --release -- fetch-modern-sources` first.");
    bail!("missing required files")
}

pub fn write_inventory(path: &Path, root: &Path, files: &[PathBuf], sort: bool) -> Result<()> {
    let mut files = files.to_vec();
    if sort {
        files.sort_by_key(|source_file| relative_to(source_file, root).unwrap_or_default());
    }
    let mut lines = Vec::new();
    for source_file in files {
        lines.push(format!(
            "{}  {}",
            sha256_file(&source_file)?,
            relative_to(&source_file, root)?
        ));
    }
    write_text(path, &format!("{}\n", lines.join("\n")))
}

pub fn write_tree_inventory(root: &Path, source_id: &str) -> Result<()> {
    let source_root = root.join("sources").join(source_id);
    let raw_root = source_root.join("raw");
    let mut files = Vec::new();
    collect_files(&raw_root, &mut files)?;
    files.sort();
    write_inventory(
        &source_root.join("source-inventory.sha256"),
        &source_root,
        &files,
        false,
    )
}

pub fn collect_files(path: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(path).with_context(|| format!("read {}", path.display()))? {
        let path = entry?.path();
        if path.is_dir() {
            collect_files(&path, files)?;
        } else if path.is_file() {
            files.push(path);
        }
    }
    Ok(())
}

pub fn file_info(path: &Path) -> Result<FileInfo> {
    Ok(FileInfo {
        sha256: sha256_file(path)?,
        size: fs::metadata(path)?.len(),
    })
}

pub fn sha256_file(path: &Path) -> Result<String> {
    let mut file = File::open(path).with_context(|| format!("read {}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let len = file.read(&mut buffer)?;
        if len == 0 {
            break;
        }
        hasher.update(&buffer[..len]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

pub fn sha256_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

pub fn write_json(path: &Path, value: &Value) -> Result<()> {
    let text = serde_json::to_string_pretty(value)?;
    write_text(path, &format!("{text}\n"))
}

pub fn write_text(path: &Path, text: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, text).with_context(|| format!("write {}", path.display()))
}

pub fn count_lines(path: &Path) -> Result<usize> {
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines().count())
}

pub fn relative_to(path: &Path, root: &Path) -> Result<String> {
    Ok(path
        .strip_prefix(root)
        .with_context(|| format!("strip {} from {}", root.display(), path.display()))?
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/"))
}

pub fn repo_relative(root: &Path, path: &Path) -> Result<String> {
    relative_to(path, root)
}
