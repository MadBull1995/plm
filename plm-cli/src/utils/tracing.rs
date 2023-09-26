use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn setup_tracing(quiet: &bool, debug: &u8) {
    if !quiet {
        // Initialize subscriber
        let subscriber = FmtSubscriber::builder()
            .with_max_level(match debug {
                0 => Level::INFO,
                1 => Level::DEBUG,
                2 => Level::TRACE,
                _ => {
                    println!("Don't be crazy");
                    Level::TRACE
                }
            })
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
    }
}
