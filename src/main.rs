#![allow(dead_code)]
//! TaskForge - CLI task management application
//!
//! Entry point for the TaskForge CLI tool.

use anyhow::Result;
use clap::Parser;
use std::env;
use std::sync::Arc;
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
use crate::commands::create_task_with_dyn;
use crate::commands::update_task_with_dyn;
use crate::config::load_config;
use crate::error::AppError;
use crate::models::Priority;
use crate::repository::{Repository, SqliteRepository};

fn main() -> Result<()> {
    // Initialize tracing subscriber for logging
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
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

    // Load configuration
    let app_config = load_config();

    // Initialize the repository
    let db_path = std::path::Path::new(&app_config.database_path);

    // Ensure the parent directory exists
    if let Some(parent) = db_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .map_err(|e| anyhow::anyhow!("Failed to create database directory: {}", e))?;
        }
    }

    let sqlite_repo = SqliteRepository::new(db_path)
        .map_err(|e| anyhow::anyhow!("Failed to create repository: {}", e))?;

    // Initialize the database schema
    sqlite_repo
        .initialize()
        .map_err(|e| anyhow::anyhow!("Failed to initialize database: {}", e))?;

    let repository: Arc<dyn Repository> = Arc::new(sqlite_repo);

    // Handle the parsed commands
    match cli.command {
        Commands::Add {
            title,
            description,
            priority,
        } => {
            let priority = match priority {
                1 => Priority::P1,
                2 => Priority::P2,
                3 => Priority::P3,
                4 => Priority::P4,
                _ => Priority::P3,
            };

            match create_task_with_dyn(repository.as_ref(), title, description, priority) {
                Ok(task) => {
                    println!("Created task: {}", task.id);
                }
                Err(e) => {
                    eprintln!("Error creating task: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::List { .. } => {
            // TODO: Implement list command
            tracing::info!("List command not yet implemented");
        }
        Commands::Show { id } => {
            match commands::get_task_with_dyn(repository.as_ref(), id.clone()) {
                Ok(task) => {
                    println!("Task Details:");
                    println!("  ID:          {}", task.id);
                    println!("  Title:       {}", task.title);
                    println!("  Description: {}", task.description.as_deref().unwrap_or("N/A"));
                    println!("  Priority:    {:?}", task.priority);
                    println!("  Status:      {:?}", task.status);
                    println!("  Created:     {}", task.created_at);
                    println!("  Updated:     {}", task.updated_at);
                    println!("  Due Date:    {}", task.due_date.map(|d| d.to_string()).unwrap_or_else(|| "N/A".to_string()));
                }
                Err(e) => match e {
                    AppError::NotFound(_) => {
                        eprintln!("Task not found");
                        std::process::exit(1);
                    }
                    _ => {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                },
            }
        }
        Commands::Edit { 
            id, 
            title, 
            description, 
            priority,
            status,
            due,
        } => {
            match update_task_with_dyn(
                repository.as_ref(),
                id,
                title,
                description,
                priority,
                status,
                due,
            ) {
                Ok(task) => {
                    println!("Updated task: {}", task.id);
                    println!("  Title:       {}", task.title);
                    println!("  Description: {}", task.description.as_deref().unwrap_or("N/A"));
                    println!("  Priority:    {:?}", task.priority);
                    println!("  Status:      {:?}", task.status);
                    println!("  Due Date:    {}", task.due_date.map(|d| d.to_string()).unwrap_or_else(|| "N/A".to_string()));
                }
                Err(e) => match e {
                    AppError::NotFound(_) => {
                        eprintln!("Task not found");
                        std::process::exit(1);
                    }
                    _ => {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                },
            }
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
