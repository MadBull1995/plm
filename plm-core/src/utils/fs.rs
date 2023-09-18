use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};

use serde::Serialize;

struct TabDelimited;

pub const plm_CONFIG_FILE: &str = ".plmrc";

#[derive(Debug)]
pub enum Entry {
    File(PathBuf),
    Directory(PathBuf, Vec<Entry>),
}

pub struct FileSystem;
use std::collections::HashMap;

use crate::Manifest;

impl FileSystem {
    pub fn create_dir(dir_path: &str) -> io::Result<()> {
        fs::create_dir_all(dir_path)
    }

    pub fn write_file(file_path: &str, contents: &str) -> io::Result<()> {
        let mut file = File::create(file_path)?;
        file.write_all(contents.as_bytes())
    }

    pub fn read_file(file_path: &str) -> io::Result<String> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn to_relative_path(absolute_path: &Path, base: &Path) -> io::Result<PathBuf> {
        let relative_path = absolute_path
            .strip_prefix(base)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Not a prefix"))?
            .to_path_buf();
        Ok(relative_path)
    }

    fn walk_dir(dir: &Path, proto_files: &mut Vec<String>) -> io::Result<Entry> {
        let mut entries = Vec::new();

        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    entries.push(Self::walk_dir(&path, proto_files)?);
                } else {
                    if path
                        .extension()
                        .and_then(|s| if s == "proto" { Some(()) } else { None })
                        .is_some()
                    {
                        proto_files.push(path.display().to_string());
                    }
                    entries.push(Entry::File(path));
                }
            }
        }

        Ok(Entry::Directory(dir.to_path_buf(), entries))
    }

    pub fn list_protos(dir_path: &PathBuf) -> io::Result<Vec<String>> {
        let mut proto_files = Vec::new();
        Self::walk_dir(dir_path, &mut proto_files)?;

        Ok(proto_files)
    }

    pub fn list_dir(dir_path: &str) -> io::Result<Vec<String>> {
        let entries = fs::read_dir(dir_path)?
            .map(|res| res.map(|e| e.path().display().to_string()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        Ok(entries)
    }

    pub fn delete_file(file_path: &str) -> io::Result<()> {
        fs::remove_file(file_path)
    }

    pub fn read_manifest(file_path: &str) -> io::Result<Manifest> {
        let content = Self::read_file(file_path)?;
        let result: Result<Manifest, serde_json::Error> = serde_json::from_str(&content);
        match result {
            Err(err) => Err(err.into()),
            Ok(manifest) => Ok(manifest),
        }
    }

    pub fn write_json<T: Serialize>(file_path: &str, value: &T) -> io::Result<()> {
        // let json = serde_json::to_value(value)?;
        let string = serde_json::to_string_pretty(&value)?;
        let mut file = File::create(file_path)?;
        file.write_all(string.as_bytes())?;
        Ok(())
    }

    pub fn write_yaml<T: Serialize>(file_path: &str, value: &T) -> io::Result<()> {
        let yaml_string = serde_yaml::to_string(value)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        let mut file = File::create(file_path)?;
        file.write_all(yaml_string.as_bytes())?;
        Ok(())
    }

    pub fn current_dir() -> io::Result<PathBuf> {
        env::current_dir()
    }

    pub fn join_paths<P: AsRef<Path>, Q: AsRef<Path>>(p: P, q: Q) -> PathBuf {
        p.as_ref().join(q.as_ref())
    }

    pub fn parse_plmrc_file(home: bool) -> io::Result<HashMap<String, String>> {
        let mut file = File::open(Self::join_paths(
            Self::current_dir().unwrap(),
            plm_CONFIG_FILE,
        ))?;
        if home {
            file = File::open(Self::join_paths(
                Self::get_home_directory().unwrap(),
                plm_CONFIG_FILE,
            ))?;
        }
        let reader = BufReader::new(file);
        let mut map = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            let trimmed_line = line.trim();

            // Skip comments and empty lines
            if trimmed_line.starts_with("#") || trimmed_line.is_empty() {
                continue;
            }

            // Split into key and value
            let parts: Vec<&str> = trimmed_line.splitn(2, '=').collect();
            if parts.len() != 2 {
                continue; // Skip malformed lines
            }

            let (key, value) = (parts[0].trim().to_string(), parts[1].trim().to_string());
            map.insert(key, value);
        }

        Ok(map)
    }

    /// Returns the current user's home directory as an `Option<String>`.
    /// Returns `None` if the home directory cannot be determined.
    pub fn get_home_directory() -> Option<String> {
        match dirs::home_dir() {
            Some(path) => Some(path.to_str()?.to_owned()),
            None => None,
        }
    }

    pub fn read_binary_file(file_path: &Path) -> io::Result<Vec<u8>> {
        let contents = fs::read(file_path)?;
        Ok(contents)
    }
}
