use std::env;
use std::fs::{self, File};
use std::path::Path;
use std::process;

fn main() {
    // OUT_DIR is set by Cargo and it's where any additional build artifacts
    // are written.
    let outdir = match env::var_os("OUT_DIR") {
        Some(outdir) => outdir,
        None => {
            eprintln!(
                "OUT_DIR environment variable not defined. \
                 Please file a bug: \
                 https://github.com/MadBull1995/plm/issues/new"
            );
            process::exit(1);
        }
    };
    fs::create_dir_all(&outdir).unwrap();

    let stamp_path = Path::new(&outdir).join("plm-stamp");
    if let Err(err) = File::create(&stamp_path) {
        panic!("failed to write {}: {}", stamp_path.display(), err);
    }
    // if let Err(err) = generate_man_page(&outdir) {
    //     eprintln!("failed to generate man page: {}", err);
    // }

    // // Use clap to build completion files.
    // let mut app = app::app();
    // app.gen_completions("rg", Shell::Bash, &outdir);
    // app.gen_completions("rg", Shell::Fish, &outdir);
    // app.gen_completions("rg", Shell::PowerShell, &outdir);
    // // Note that we do not use clap's support for zsh. Instead, zsh completions
    // // are manually maintained in `complete/_rg`.

    // Make the current git hash available to the build.
    if let Some(rev) = git_revision_hash() {
        println!("cargo:rustc-env=RIPGREP_BUILD_GIT_HASH={}", rev);
    }
    // Embed a Windows manifest and set some linker options. The main reason
    // for this is to enable long path support on Windows. This still, I
    // believe, requires enabling long path support in the registry. But if
    // that's enabled, then this will let ripgrep use C:\... style paths that
    // are longer than 260 characters.
    set_windows_exe_options();
}

fn set_windows_exe_options() {
    static MANIFEST: &str = "pkg/windows/Manifest.xml";

    let Ok(target_os) = env::var("CARGO_CFG_TARGET_OS") else {
        return;
    };
    let Ok(target_env) = env::var("CARGO_CFG_TARGET_ENV") else {
        return;
    };
    if !(target_os == "windows" && target_env == "msvc") {
        return;
    }

    let Ok(mut manifest) = env::current_dir() else {
        return;
    };
    manifest.push(MANIFEST);
    let Some(manifest) = manifest.to_str() else {
        return;
    };

    println!("cargo:rerun-if-changed={}", MANIFEST);
    // Embed the Windows application manifest file.
    println!("cargo:rustc-link-arg-bin=plm=/MANIFEST:EMBED");
    println!("cargo:rustc-link-arg-bin=plm=/MANIFESTINPUT:{manifest}");
    // Turn linker warnings into errors. Helps debugging, otherwise the
    // warnings get squashed (I believe).
    println!("cargo:rustc-link-arg-bin=plm=/WX");
}

fn git_revision_hash() -> Option<String> {
    let result = process::Command::new("git")
        .args(["rev-parse", "--short=10", "HEAD"])
        .output();
    result.ok().and_then(|output| {
        let v = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if v.is_empty() {
            None
        } else {
            Some(v)
        }
    })
}
