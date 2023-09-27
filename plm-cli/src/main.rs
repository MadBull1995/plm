use anyhow::Context;
use human_panic::setup_panic;
use log::{debug, LevelFilter};
use plm_cli::{
    commands,
    helpers::{get_global_plmrc_path, get_manifest_from_file},
    parse_cli,
    tracing::setup_tracing,
    utils::{
        self,
        configs::CliConfigs,
        errors::{PlmError, PlmResult},
        lock::ProtoLock,
        prompter::Prompter,
    },
    Cli, Commands,
};
use plm_core::FileSystem;
use std::{
    collections::HashMap, io::Write as ioWrite
};
use tokio::{
    signal,
    sync::mpsc,
};

#[tokio::main]
async fn main() -> PlmResult<()> {
    setup_panic!();
    let (_shutdown_send, mut shutdown_recv) = mpsc::channel::<bool>(1);

    tokio::spawn(async move {
        tokio::select! {
            _ = signal::ctrl_c() => {
                Prompter::warning("User interrupted");
                std::process::exit(exitcode::SOFTWARE);
            },
            _ = shutdown_recv.recv() => {
                Prompter::error("Aborting process");
                std::process::exit(exitcode::SOFTWARE);
            },
        }
    });

    let args = parse_cli();
    setup_tracing(&args.quiet, &args.debug);
    setup_prompter(args.debug, args.quiet);

    Prompter::info("starting plm process");

    let mut cfgs = CliConfigs::new();
    cfgs.load_plmrc_files()?;

    let result = utils::time_it("Process", || async {
        process_commands(args, &mut cfgs).await
    })
    .await;

    match result {
        Ok(_) => {
            // Successfully processed the command
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            std::process::exit(exitcode::DATAERR);
        }
    }

    std::process::exit(exitcode::OK);
}

// async fn process() -> PlmResult<()> {
//     sleep(Duration::from_millis(500)).await;
//     Prompter::task(1, 7, "Adding WASM target...");

//     Prompter::task(2, 7, "Compiling to WASM...");

//     Prompter::task(3, 7, "Creating a pkg directory...");
//     sleep(Duration::from_millis(500)).await;

//     let mut downloaded = 0;
//     let total_size = 231231231;

//     let pb = indicatif::ProgressBar::new(total_size);
//     pb.set_style(
//         indicatif::ProgressStyle::with_template(
//             "{msg}\n[{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})",
//         )
//         .unwrap()
//         .with_key(
//             "eta",
//             |state: &indicatif::ProgressState, w: &mut dyn Write| {
//                 write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
//             },
//         )
//         .progress_chars("#-"),
//     );

//     while downloaded < total_size {
//         let new = std::cmp::min(downloaded + 223211, total_size);
//         downloaded = new;
//         // pb.println(format!("[+] finished #{}", downloaded));
//         pb.set_position(new);
//         pb.set_message(
//             format!(">{:3}%", 100 * downloaded / total_size)
//                 .dimmed()
//                 .to_string(),
//         );
//         sleep(Duration::from_millis(12)).await;
//     }

//     pb.finish_with_message("> downloaded");

//     Ok(())
// }

async fn process_commands(args: Cli, cfgs: &mut CliConfigs) -> anyhow::Result<()> {
    match args.command {
        // <-------- Init-------------->
        Commands::Init(init) => {
            if check_lib_exists() {
                Prompter::warning("seems like a library is alreasy initiated on current directory");
            } else {
                let proto_lock_path = proto_lock_path(cfgs);
                let proto_lock = ProtoLock::default();
                proto_lock.to_file(proto_lock_path)?;
                debug!("{:#?}", init);
                commands::init::init_command(&init)
                    .await
                    .with_context(|| "init command errored".to_string())?;
            }
        }

        // <-------- Install ---------->
        Commands::Install(install) => {
            let proto_lock_path = proto_lock_path(cfgs);
            let mut manifest = get_manifest_from_file()?;

            Prompter::task(1, 6, "resolving proto-lock.json file");
            let mut proto_lock =
                ProtoLock::from_file(proto_lock_path.clone()).unwrap_or(ProtoLock::default());
            if let Some(library) = proto_lock.find_library(install.name.clone()) {
                // Handle logic if package is already installed, perhaps prompt for update or exit
                Prompter::warning(&format!(
                    "Package {} already exists under current lib: {}",
                    library.name, manifest.name
                ));

                Prompter::info(&format!("Try to run: $ plm update {}", library.name));
            } else {
                commands::install::install_command(
                    install,
                    &mut manifest,
                    FileSystem::current_dir().unwrap().as_path(),
                    &proto_lock_path,
                    &mut proto_lock,
                    cfgs.registry.clone(),
                    cfgs.clone().token.unwrap_or_default(),
                )
                .await
                .with_context(|| "install command errored".to_string())?;
            }
        }

        // <-------- Uninstall -------->
        Commands::Uninstall(_uninstall) => todo!(),

        // <-------- Publish ---------->
        Commands::Publish(_publish) => {
            let manifest = get_manifest_from_file()?;

            commands::publish::publish_command(
                manifest,
                cfgs.clone(),
                cfgs.clone().token.unwrap_or_default(),
            )
            .await
            .with_context(|| "publish command errored".to_string())?;
        }

        // <-------- Login ------------>
        Commands::Login(login) => {
            commands::login::login_command(
                cfgs,
                &login.user,
                &login.password,
                cfgs.registry.clone(),
            )
            .await
            .with_context(|| "login command errored".to_string())?;
        }

        // <-------- Config ----------->
        Commands::Config(cfg) => {
            if let Some(cmd) = cfg.command {
                match cmd {
                    plm_cli::ConfigCommand::Get { key } => match get_plmrc_file().await {
                        Some(rc) => {
                            let kv = rc.get_key_value(&key);
                            match kv {
                                Some((key, value)) => {
                                    Prompter::normal(format!("{} = {:#?}", key, value).as_str());
                                }
                                None => {
                                    Prompter::warning(format!("Not found any configuration for -> '{}', under {:?} file", key, get_global_plmrc_path()).as_str());
                                }
                            }
                        }
                        None => {
                            Prompter::warning("No configuration file found.");
                        }
                    },
                    plm_cli::ConfigCommand::Set { key, value } => {
                        // let mut config = cfg().await.unwrap_or_else(|| HashMap::new());
                        // config.insert(key.clone(), value.clone());
                        if key == *"registry" {
                            cfgs.registry = value;
                        } else {
                            // cfgs.to_json()
                            // serde_json::to_string(cfgs);
                            Prompter::warning(&format!("cant set '{}', use other commands to interact with this specific config", key));
                        }

                        cfgs.write_plmrc_file()?;
                    }
                    plm_cli::ConfigCommand::Show { json: _ } => {
                        cfgs.to_json();
                    }
                }
            }
        }
    }
    Ok(())
}

// async fn handle_shutdown(tx: mpsc::Sender<bool>) -> PlmResult<()> {
//     tx.send(true)
//         .await
//         .map_err(|_err| PlmError::InternalError("Failed to send shutdown signal".to_string()))?;

//     Ok(())
// }

async fn get_plmrc_file() -> Option<HashMap<String, String>> {
    let dot_plmrc = FileSystem::parse_plmrc_file(true).map_err(|err| {
        PlmError::InternalError(format!("Failed to load .plmrc: {}", err))
    });
    let mut plmrc: Option<HashMap<String, String>> = None;
    match dot_plmrc {
        Ok(cfg) => {
            plmrc = Some(cfg.clone());
        }
        Err(err) => {
            Prompter::error(err.to_string().as_str());
        }
    }
    plmrc
}

fn proto_lock_path(cfgs: &CliConfigs) -> std::path::PathBuf {
    FileSystem::join_paths(cfgs.clone().current_dir, "proto-lock.json")
}

fn check_lib_exists() -> bool {
    FileSystem::file_exists(
        FileSystem::join_paths(FileSystem::current_dir().unwrap(), "proto-package.json")
            .to_str()
            .unwrap(),
    )
}

fn setup_prompter(debug: u8, quiet: bool) {
    let mut log_builder = env_logger::builder();
    if quiet {
        log_builder.filter_level(log::LevelFilter::Off);
    } else {
        log_builder.filter_level(match debug {
            1 => LevelFilter::Debug,
            2 => LevelFilter::Trace,
            _ => LevelFilter::Info,
        });
    }
    log_builder
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();
}
