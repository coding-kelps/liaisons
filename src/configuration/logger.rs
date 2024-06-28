use tracing::subscriber::SetGlobalDefaultError;
use tracing_subscriber::{filter::LevelFilter, filter::LevelParseError};
use std::str::FromStr;
use crate::configuration::{settings, LogOutput};
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("failed to parse logger level filter")]
    LevelParseError(#[from] LevelParseError),

    #[error("failed to set logger as global default: {0}")]
    SetGlobalDefaultError(#[from] SetGlobalDefaultError)
}

const LOGGING_DIR: &str = "/var/log/liaisons";
const LOG_FILE_PREFIX: &str = "liaisons.log";

// Setup the env_logger logger from a Log configuration
pub fn setup(cfg: &settings::Log) -> Result<(), Error> {
    let filter = LevelFilter::from_str(cfg.level.to_lowercase().as_str())?;

    let mut subscriber_builder = tracing_subscriber::fmt()
        // Use a more compact, abbrievated log format
        .compact()
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Set level filter
        .with_max_level(filter)
        // Display time
        .with_timer(tracing_subscriber::fmt::time::ChronoUtc::rfc_3339());

    let writer = match &cfg.output {
        LogOutput::Stdout => {
            subscriber_builder = subscriber_builder.with_ansi(true);

            let (non_blocking, _guard) =
                tracing_appender::non_blocking(std::io::stdout());

            non_blocking
        },
        LogOutput::Stderr => {
            subscriber_builder = subscriber_builder.with_ansi(true);

            let (non_blocking, _guard) =
                tracing_appender::non_blocking(std::io::stderr());

            non_blocking
        },
        LogOutput::Default => {
            subscriber_builder = subscriber_builder.with_ansi(false);

            let file_appender =
                tracing_appender::rolling::daily(LOGGING_DIR, LOG_FILE_PREFIX);

            let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

            non_blocking
        },
    };

    let subscriber = subscriber_builder        
        // Set log output (either it be a file or an os output)
        .with_writer(writer)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
