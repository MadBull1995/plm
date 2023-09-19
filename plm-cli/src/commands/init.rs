use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use tracing::debug;

use crate::utils::{
    self,
    errors::{PlmResult, PlmError},
    prompter::{plm_theme, Prompter},
};
use plm_core::FileSystem;
pub async fn init_command(verbose: &bool) -> PlmResult<()> {
    let mut manifest = plm_core::Manifest::default();


    let project_name: String = Input::with_theme(&plm_theme())
        .with_prompt("Enter your project name")
        .interact()
        .unwrap();
    manifest.name = project_name.clone();

    let choices = &["MIT", "GPL-3.0", "APACHE-2.0", "Unlicense"];
    let selection = Select::with_theme(&plm_theme())
        .with_prompt("Choose a license")
        .default(0)
        .items(&choices[..])
        .interact()
        .unwrap();

    let license = choices[selection];
    manifest.license = license.to_owned();

    let initialize_git: bool = Confirm::with_theme(&plm_theme())
        .with_prompt("Initialize a git repository?")
        .interact()
        .unwrap();

    Prompter::normal(&format!("Project Name: {}", project_name));
    Prompter::normal(&format!("License: {}", license));
    Prompter::normal(&format!("Initialize git: {}", initialize_git));

    manifest.version = "0.0.1".to_string();
    manifest.description = "Awesome Protobuf Package".to_string();
    manifest.src_dir = "protos".to_string();

    let lib_dir = FileSystem::current_dir().unwrap();
    let path = FileSystem::join_paths(lib_dir.clone(), "proto-package.json");
    FileSystem::write_json(path.to_str().unwrap(), &manifest)
        .map_err(|err| panic!("error writing file"))?;

    Prompter::verbose("creating /.plm");
    let dot_plm_dir = FileSystem::join_paths(lib_dir.clone(), ".plm");
    FileSystem::create_dir(
            dot_plm_dir.clone()
            .to_str()
            .unwrap(),
    )
    .map_err(|err| PlmError::InternalError("Failed to create directory".to_string()))?;
    Prompter::verbose("creating /.plm/builds");
    FileSystem::create_dir(
        FileSystem::join_paths(dot_plm_dir.clone(), "builds")
            .to_str()
            .unwrap(),
    )
    .map_err(|err| PlmError::InternalError("Failed to create directory".to_string()))?;
    if *verbose {
        dbg!(manifest);
    }

    Ok(())
}
