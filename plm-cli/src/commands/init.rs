use std::path::{Path, PathBuf};

use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use tracing::debug;

use crate::{utils::{
    self,
    errors::{PlmError, PlmResult},
    prompter::{plm_theme, Prompter},
}, Init};
use plm_core::FileSystem;
pub async fn init_command(init: &Init, verbose: &bool) -> PlmResult<()> {
    let mut manifest = plm_core::Manifest::default();
   
    let project_name = if let Some(name) = &init.library_name {
        name.clone()
    } else {
        Input::with_theme(&plm_theme())
            .with_prompt("Enter your project name")
            .interact().map_err(|e| PlmError::InternalError(e.to_string()))?
    };
    manifest.name = project_name.clone();

    let available_licenses = &["MIT", "GPL-3.0", "APACHE-2.0", "Unlicense"];
    
    let license = if let Some(lic) = &init.license {
        match lic {
            crate::License::MIT => available_licenses[0].to_string(),
            crate::License::GPL => available_licenses[1].to_string(),
            crate::License::APACHE2 => available_licenses[2].to_string(),
            crate::License::UNLICENSE => available_licenses[3].to_string()
        }
    } else {
        let selection = Select::with_theme(&plm_theme())
            .with_prompt("Choose a license")
            .default(0)
            .items(&available_licenses[..])
            .interact().map_err(|e| PlmError::InternalError(e.to_string()))?;
        available_licenses[selection].to_owned()
    };
    
    manifest.license = license.clone();

    manifest.version = init.version.clone().unwrap_or_else(|| "0.0.1".to_string());
    manifest.description = init.description.clone().unwrap_or_else(|| "Awesome Protobuf Package".to_string());;
    manifest.src_dir = init.src_dir.clone().unwrap_or_else(|| "".to_string());
    
    // Gets a value for the "list" argument
    manifest.exclude = if let Some(list_str) = init.clone().exclude {
        let list_vec: Vec<&str> = list_str.split(',').collect();
        list_vec.iter().map(|v| v.to_string()).collect()
    } else {
        vec![]
    };
    log_summary(&project_name, &license, &manifest.version, &manifest.description);
    
    let lib_dir = FileSystem::current_dir().map_err(|e| PlmError::InternalError(e.to_string()))?;
    let manifest_path = FileSystem::join_paths(lib_dir.clone(), "proto-package.json");
    write_manifest(&manifest_path.to_str().unwrap(), &manifest)?;

    let dot_plm_dir = create_directory(&lib_dir, ".plm")?;
    create_directory(&dot_plm_dir, "builds")?;
    Prompter::verbose("creating /proto_modules");
    plm_core::library::store::LibraryStore::create()
        .await
        .map_err(|err| PlmError::InternalError(err.to_string()))?;


    // let lib_dir = FileSystem::current_dir().unwrap();
    // let path = FileSystem::join_paths(lib_dir.clone(), "proto-package.json");
    // FileSystem::write_json(path.to_str().unwrap(), &manifest)
    //     .map_err(|err| panic!("error writing file"))?;

    // Prompter::verbose("creating /.plm");
    // let dot_plm_dir = FileSystem::join_paths(lib_dir.clone(), ".plm");
    // FileSystem::create_dir(dot_plm_dir.clone().to_str().unwrap())
    //     .map_err(|err| PlmError::InternalError("Failed to create directory".to_string()))?;
    // Prompter::verbose("creating /.plm/builds");
    // FileSystem::create_dir(
    //     FileSystem::join_paths(dot_plm_dir.clone(), "builds")
    //         .to_str()
    //         .unwrap(),
    // )
    // .map_err(|err| PlmError::InternalError("Failed to create directory".to_string()))?;
    // Prompter::verbose("creating /proto_modules");
    // plm_core::library::store::LibraryStore::create()
    //     .await
    //     .map_err(|err| PlmError::InternalError(err.to_string()))?;

    Ok(())
}


fn log_summary(project_name: &str, license: &str, version: &str, description: &str) {
    Prompter::normal(&format!("Project Name: {}", project_name));
    Prompter::normal(&format!("License: {}", license));
    Prompter::normal(&format!("Version: {}", version));
    Prompter::normal(&format!("Description: {}", description));
}

fn write_manifest(path: &str, manifest: &plm_core::Manifest) -> PlmResult<()> {
    FileSystem::write_json(path, manifest)
        .map_err(|_| PlmError::InternalError("Error writing file".to_string()))
}


fn create_directory(parent: &Path, dir: &str) -> PlmResult<PathBuf> {
    let new_dir = FileSystem::join_paths(parent.clone(), dir);
    Prompter::verbose(&format!("creating {}", dir));
    FileSystem::create_dir(new_dir.clone().to_str().unwrap())
        .map_err(|_| PlmError::InternalError(format!("Failed to create directory {}", dir)))?;
    Ok(new_dir)
}
