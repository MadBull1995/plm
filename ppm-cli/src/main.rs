use std::{time::Duration, fmt::Write, process::ExitCode};

use ppm_cli::{parse_cli, Commands, utils::{errors::PPMResult, prompter::Prompter, self}, commands};
use tokio::{time::sleep, signal, sync::{mpsc, oneshot}};
use colored::*;
#[tokio::main]
async fn main() {
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
    Prompter::info("starting ppm process");
    utils::time_it("Process", || async {
        match args.command {
            Commands::Install(install) => {
                println!("{:?}", install);
            },
            Commands::Uninstall(uninstall) => todo!(),
            Commands::Publish(publish) => todo!(),
            Commands::Init(init) => {
                commands::init::init_command(&init.verbose).await;
                shutdown_send.send(true).await;
            },
            Commands::Login(login) => {
                println!("{:?}", login);
            },
        }

        let result = process().await;
        match result {
            Ok(_) => {
                
            }
            // Err(CustomError::CantReadConfig(e)) => {
            //     eprintln!("Error: {}", e);
            //     std::process::exit(exitcode::CONFIG);
            // }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(exitcode::DATAERR);
            }
        }
    }).await;
    
    std::process::exit(exitcode::OK);

}

async fn process() -> PPMResult<()> {
    
    sleep(Duration::from_millis(500)).await;
    Prompter::task(1, 7, "Adding WASM target...");

    Prompter::task(2, 7, "Compiling to WASM...");

    Prompter::task(3, 7, "Creating a pkg directory...");
    sleep(Duration::from_millis(500)).await;
    
    let mut downloaded = 0;
    let total_size = 231231231;

    let pb = indicatif::ProgressBar::new(total_size);
    pb.set_style(indicatif::ProgressStyle::with_template("{msg}\n[{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &indicatif::ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#-"));
    
    while downloaded < total_size {
        let new = std::cmp::min(downloaded + 223211, total_size);
        downloaded = new;
        // pb.println(format!("[+] finished #{}", downloaded));
        pb.set_position(new);
        pb.set_message(format!(">{:3}%", 100 * downloaded / total_size).dimmed().to_string());
        sleep(Duration::from_millis(12)).await;

    }

    pb.finish_with_message("> downloaded");

    Ok(())
}