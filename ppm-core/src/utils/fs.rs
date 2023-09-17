use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use serde::Serialize;

struct TabDelimited;

pub struct FileSystem;

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

    pub fn list_dir(dir_path: &str) -> io::Result<Vec<String>> {
        let entries = fs::read_dir(dir_path)?
            .map(|res| res.map(|e| e.path().display().to_string()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        Ok(entries)
    }

    pub fn delete_file(file_path: &str) -> io::Result<()> {
        fs::remove_file(file_path)
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
}
