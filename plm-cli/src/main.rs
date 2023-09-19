use std::{collections::HashMap, fmt::Write, process::ExitCode, time::Duration};

use colored::*;
use human_panic::setup_panic;
use plm_cli::{
    commands,
    helpers::{get_global_plmrc_path, get_manifest_from_file},
    parse_cli,
    utils::{
        self,
        configs::CliConfigs,
        errors::{PlmResult, PlmError},
        prompter::Prompter, lock::ProtoLock,
    },
    Cli, Commands,
};
use plm_core::FileSystem;
use tokio::{
    signal,
    sync::{mpsc, oneshot},
    time::sleep,
};
#[tokio::main]
async fn main() -> PlmResult<()> {
    setup_panic!();
    let (shutdown_send, mut shutdown_recv) = mpsc::channel::<bool>(1);

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

    Prompter::info("starting plm process");

    let mut cfgs = CliConfigs::new();
    cfgs.load_plmrc_files()?;

    let result = utils::time_it("Process", || async {
        process_commands(args, cfgs, shutdown_send).await
    })
    .await;

    match result {
        Ok(_) => {
            // Successfully processed the command
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(exitcode::DATAERR);
        }
    }

    std::process::exit(exitcode::OK);
}

async fn process() -> PlmResult<()> {
    sleep(Duration::from_millis(500)).await;
    Prompter::task(1, 7, "Adding WASM target...");

    Prompter::task(2, 7, "Compiling to WASM...");

    Prompter::task(3, 7, "Creating a pkg directory...");
    sleep(Duration::from_millis(500)).await;

    let mut downloaded = 0;
    let total_size = 231231231;

    let pb = indicatif::ProgressBar::new(total_size);
    pb.set_style(
        indicatif::ProgressStyle::with_template(
            "{msg}\n[{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})",
        )
        .unwrap()
        .with_key(
            "eta",
            |state: &indicatif::ProgressState, w: &mut dyn Write| {
                write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
            },
        )
        .progress_chars("#-"),
    );

    while downloaded < total_size {
        let new = std::cmp::min(downloaded + 223211, total_size);
        downloaded = new;
        // pb.println(format!("[+] finished #{}", downloaded));
        pb.set_position(new);
        pb.set_message(
            format!(">{:3}%", 100 * downloaded / total_size)
                .dimmed()
                .to_string(),
        );
        sleep(Duration::from_millis(12)).await;
    }

    pb.finish_with_message("> downloaded");

    Ok(())
}

async fn process_commands(
    args: Cli,
    cfgs: CliConfigs,
    shutdown_send: mpsc::Sender<bool>,
) -> Result<(), PlmError> {
    match args.command {
        // <-------- Install ---------->
        Commands::Install(install) => {
            println!("{:?}", install);
        }

        // <-------- Uninstall -------->
        Commands::Uninstall(uninstall) => todo!(),

        // <-------- Publish ---------->
        Commands::Publish(publish) => {
            let manifest = get_manifest_from_file(&publish.verbose)
                .map_err(|err| return PlmError::InternalError);
            match manifest {
                Ok(m) => {
                    commands::publish::publish_command(m, cfgs.clone(), publish.verbose).await?;
                }
                Err(err) => {
                    handle_shutdown(shutdown_send.clone()).await?;
                }
            }
        }

        // <-------- Init-------------->
        Commands::Init(init) => {
            let proto_lock = ProtoLock::default();
            proto_lock.to_file(FileSystem::join_paths(cfgs.current_dir, "proto-lock.json"))?;

            commands::init::init_command(&false).await?;
            handle_shutdown(shutdown_send.clone()).await?;
        }

        // <-------- Login ------------>
        Commands::Login(login) => {
            println!("{:?}", login);
        }

        // <-------- Config ----------->
        Commands::Config(cfg) => {
            

            match cfg.action {
                plm_cli::ConfigAction::Get => {
                    match get_plmrc_file(shutdown_send.clone()).await {
                        Some(rc) => {
                            let kv = rc.get_key_value(&cfg.key);
                            match kv {
                                Some((key, value)) => {
                                    Prompter::normal(format!("{} = {:#?}", key, value).as_str());
                                }
                                None => {
                                    Prompter::warning(format!("Not found any configuration for -> '{}', under {:?} file", cfg.key, get_global_plmrc_path()).as_str());
                                }
                            }
                        }
                        None => handle_shutdown(shutdown_send.clone()).await?,
                    }
                }
                plm_cli::ConfigAction::Set => {
                    todo!()
                }
            }
        }
    }
    Ok(())
}

async fn handle_shutdown(tx: mpsc::Sender<bool>) -> PlmResult<()> {
    tx.send(true)
        .await
        .map_err(|err| PlmError::InternalError("Failed to send shutdown signal".to_string()))?;

    Ok(())
}

async fn get_plmrc_file(shutdown_send: mpsc::Sender<bool>) -> Option<HashMap<String, String>> {
    let dot_plmrc = FileSystem::parse_plmrc_file(true)
                .map_err(|err| return PlmError::InternalError("Failed to load .plmrc".to_string()));
    let mut plmrc: Option<HashMap<String, String>> = None;
    match dot_plmrc {
        Ok(cfg) => {
            plmrc = Some(cfg.clone());
        }
        Err(err) => {
            Prompter::error(err.to_string().as_str());
            handle_shutdown(shutdown_send.clone()).await.expect("failed to send shutdown signal");
        }
    }
    plmrc
}