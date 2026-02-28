#![allow(dead_code)]
//! TaskForge - CLI task management application
//!
//! Entry point for the TaskForge CLI tool.

use anyhow::Result;
use clap::Parser;
use std::env;
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

mod cli;
mod commands;
mod config;
mod error;
mod filter;
mod models;
mod repository;
mod tag;
mod task;

use crate::cli::{Cli, Commands};

fn main() -> Result<()> {
    // Initialize tracing subscriber for logging
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    info!("TaskForge starting up");

    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // If no arguments provided (besides program name), exit successfully
    // This allows `cargo run` to exit cleanly
    if args.len() <= 1 {
        return Ok(());
    }

    // Parse command line arguments
    // Cli::parse() will handle --help and --version automatically
    // and display appropriate messages
    let cli = Cli::parse();

    // Handle the parsed commands
    match cli.command {
        Commands::Add { .. } => {
            // TODO: Implement add command
            tracing::info!("Add command not yet implemented");
        }
        Commands::List { .. } => {
            // TODO: Implement list command
            tracing::info!("List command not yet implemented");
        }
        Commands::Show { .. } => {
            // TODO: Implement show command
            tracing::info!("Show command not yet implemented");
        }
        Commands::Edit { .. } => {
            // TODO: Implement edit command
            tracing::info!("Edit command not yet implemented");
        }
        Commands::Delete { .. } => {
            // TODO: Implement delete command
            tracing::info!("Delete command not yet implemented");
        }
        Commands::Complete { .. } => {
            // TODO: Implement complete command
            tracing::info!("Complete command not yet implemented");
        }
        Commands::Reopen { .. } => {
            // TODO: Implement reopen command
            tracing::info!("Reopen command not yet implemented");
        }
        Commands::Tag { .. } => {
            // TODO: Implement tag command
            tracing::info!("Tag command not yet implemented");
        }
    }

    Ok(())
}
