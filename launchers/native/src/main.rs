use log::{info, LevelFilter};

fn main() {
    #[cfg(debug_assertions)]
    {
        // Initialize logger
        match std::env::var("RUST_LOG") {
            Ok(level) => pretty_env_logger::formatted_builder()
                .parse_filters(&level)
                .init(),
            Err(_) => pretty_env_logger::formatted_builder()
                .filter_level(LevelFilter::Trace)
                .init(),
        }
    }

    info!("Starting...");
    my_game::app().run();
}
