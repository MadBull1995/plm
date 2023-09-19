use std::path::Path;

use plm_core::Manifest;

use crate::{utils::{errors::PlmResult, lock::{ProtoLock, Library}, prompter::Prompter}, Install};


pub async fn install_command(install: Install, manifest:&mut Manifest, proto_lock_path: &Path, proto_lock: &mut ProtoLock) -> PlmResult<()> {
    // TODO: Fetch Library from Registry
    Prompter::task(3, 6, "fetching library from registry");


    Prompter::task(4, 6, "resolving dependencies");
    // Resolve library deps
    let resolved_deps = proto_lock.resolve_dependencies(&install.name)?;

    // TODO: Download Dependencies
    Prompter::task(5, 6, "fetching dependencies");

    // Add library to the proto-lock file
    let installed_lib = Library {
        name: install.name.to_string(),
        version: "0.0.1".to_string(),
        dependencies: resolved_deps,
    };
    
    Prompter::task(6, 6, "updating proto-lock.json file");
    proto_lock.add_library(installed_lib.clone());
    proto_lock.validate()?;
    proto_lock.to_file(proto_lock_path)?;
    dbg!(proto_lock);

    manifest.dependencies.insert(installed_lib.clone().name, installed_lib.version);
    println!("{:?}", install);

    Ok(())
}