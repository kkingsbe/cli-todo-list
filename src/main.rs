#![allow(dead_code)]
//! TaskForge - CLI task management application
//!
//! Entry point for the TaskForge CLI tool.

use anyhow::Result;
use clap::Parser;
use std::env;

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
            println!("Add command not yet implemented");
        }
        Commands::List { .. } => {
            // TODO: Implement list command
            println!("List command not yet implemented");
        }
        Commands::Show { .. } => {
            // TODO: Implement show command
            println!("Show command not yet implemented");
        }
        Commands::Edit { .. } => {
            // TODO: Implement edit command
            println!("Edit command not yet implemented");
        }
        Commands::Delete { .. } => {
            // TODO: Implement delete command
            println!("Delete command not yet implemented");
        }
        Commands::Complete { .. } => {
            // TODO: Implement complete command
            println!("Complete command not yet implemented");
        }
        Commands::Reopen { .. } => {
            // TODO: Implement reopen command
            println!("Reopen command not yet implemented");
        }
        Commands::Tag { .. } => {
            // TODO: Implement tag command
            println!("Tag command not yet implemented");
        }
    }

    Ok(())
}
