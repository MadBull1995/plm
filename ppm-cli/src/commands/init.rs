use dialoguer::{Input, theme::ColorfulTheme, Select, Confirm};
use tracing::debug;

use crate::{utils::{errors::PPMResult, prompter::{Prompter, ppm_theme}, self}};
use ppm_core::FileSystem;
pub async fn init_command(verbose: &bool) -> PPMResult<()> {
    let mut manifest = ppm_core::Manifest::default();
    
    let project_name: String = Input::with_theme(&ppm_theme())
        .with_prompt("Enter your project name")
        .interact()
        .unwrap();
    manifest.name = project_name.clone();

    let choices = &["MIT", "GPL-3.0", "APACHE-2.0", "Unlicense"];
    let selection = Select::with_theme(&ppm_theme())
        .with_prompt("Choose a license")
        .default(0)
        .items(&choices[..])
        .interact()
        .unwrap();

    let license = choices[selection];
    manifest.license = license.to_owned();

    let initialize_git: bool = Confirm::with_theme(&ppm_theme())
        .with_prompt("Initialize a git repository?")
        .interact()
        .unwrap();

    Prompter::normal(&format!("Project Name: {}", project_name));
    Prompter::normal(&format!("License: {}", license));
    Prompter::normal(&format!("Initialize git: {}", initialize_git));
    
    manifest.version = "0.0.1".to_string();
    manifest.description = "Awesome Protobuf Package".to_string();
    manifest.src_dir = "protos".to_string();

    let path = FileSystem::join_paths(FileSystem::current_dir().unwrap(), "proto-package.json");
    FileSystem::write_json(path.to_str().unwrap(), &manifest).map_err(|err| panic!("error writing file"));

    if *verbose {
        dbg!(manifest);
    }

    Ok(())
}