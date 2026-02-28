//! TaskForge - CLI task management application
//!
//! Entry point for the TaskForge CLI tool.

use clap::Parser;
use taskforge::AppError;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> Result<(), AppError> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("TaskForge starting...");

    // Parse CLI arguments
    let _cli = taskforge::Cli::parse();

    info!("TaskForge completed successfully");
    Ok(())
}
