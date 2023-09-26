use plm_core::FileSystem as fs;
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::errors::{PlmError, PlmResult};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dependency {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Library {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProtoLock {
    pub libraries: Vec<Library>,
}

impl ProtoLock {
    // Read from lock file
    pub fn from_file<P: AsRef<Path>>(path: P) -> PlmResult<Self> {
        let file_content = fs::read_file(path.as_ref().to_str().unwrap())
            .map_err(|err| PlmError::FileSystemError(err))?;
        let proto_lock: ProtoLock = serde_json::from_str(&file_content)
            .map_err(|err| PlmError::SerializationError(err.into()))?;
        Ok(proto_lock)
    }

    // Write to lock file
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> PlmResult<()> {
        let file_content = serde_json::to_string_pretty(self)
            .map_err(|err| PlmError::SerializationError(err.into()))?;
        fs::write_file(path.as_ref().to_str().unwrap(), &file_content)
            .map_err(|err| PlmError::FileSystemError(err))?;
        Ok(())
    }

    // Add a new library
    pub fn add_library(&mut self, lib: Library) {
        self.libraries.push(lib);
    }

    // Remove a library by name
    pub fn remove_library(&mut self, lib_name: &str) {
        self.libraries.retain(|lib| lib.name != lib_name);
    }

    // Update an existing library
    pub fn update_library(&mut self, updated_lib: Library) {
        for lib in &mut self.libraries {
            if lib.name == updated_lib.name {
                *lib = updated_lib.clone();
                return;
            }
        }
    }

    // Find a library by its name
    pub fn find_library(&self, lib_name: Option<String>) -> Option<&Library> {
        if lib_name.is_some() {
            self.libraries
                .iter()
                .find(|&lib| lib.name == lib_name.clone().unwrap().as_str())
        } else {
            None
        }
    }

    // Resolve a library's dependencies recursively
    pub fn resolve_dependencies(&self, lib_name: String) -> PlmResult<Vec<Dependency>> {
        
        Ok(Vec::new()) // Placeholder
    }

    // Validate the entire lock file, e.g., for cyclic dependencies
    pub fn validate(&self) -> PlmResult<()> {
        
        Ok(()) // Placeholder
    }
}
