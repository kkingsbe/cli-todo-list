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
use crate::commands::{
    add_tag_to_task_with_dyn, create_task_with_dyn, get_or_create_tag_with_dyn,
};
use crate::config::load_config;
use crate::error::AppError;
use crate::filter::{SortOrder, TaskFilter, TaskSort, TaskSortField};
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
            tag,
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
                    // Process tags if any were provided
                    for tag_name in tag {
                        match get_or_create_tag_with_dyn(repository.as_ref(), tag_name) {
                            Ok(tag) => {
                                if let Err(e) = add_tag_to_task_with_dyn(
                                    repository.as_ref(),
                                    task.id.clone(),
                                    tag.id,
                                ) {
                                    eprintln!("Warning: Failed to add tag to task: {}", e);
                                }
                            }
                            Err(e) => {
                                eprintln!("Warning: Failed to create/get tag: {}", e);
                            }
                        }
                    }
                    println!("Created task: {}", task.id);
                }
                Err(e) => {
                    eprintln!("Error creating task: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::List {
            status,
            priority,
            search,
            sort_by,
            order,
            limit,
            ..
        } => {
            // Parse sort field
            let sort_field = match sort_by.as_str() {
                "created" => TaskSortField::CreatedAt,
                "updated" => TaskSortField::UpdatedAt,
                "priority" => TaskSortField::Priority,
                "due" => TaskSortField::DueDate,
                "title" => TaskSortField::Title,
                _ => TaskSortField::CreatedAt,
            };

            // Parse sort order
            let sort_order = match order.as_str() {
                "asc" => SortOrder::Ascending,
                "desc" => SortOrder::Descending,
                _ => SortOrder::Descending,
            };

            // Build filter
            let mut filter = TaskFilter::new();

            // Parse status filter
            if let Some(ref status_str) = status {
                match status_str.to_lowercase().as_str() {
                    "complete" | "completed" => {
                        filter = filter.with_status(crate::models::Status::Completed);
                    }
                    "incomplete" | "open" => {
                        filter = filter.with_status(crate::models::Status::Incomplete);
                    }
                    _ => {}
                }
            }

            // Parse priority filter
            if let Some(p) = priority {
                let prio = match p {
                    1 => crate::models::Priority::P1,
                    2 => crate::models::Priority::P2,
                    3 => crate::models::Priority::P3,
                    4 => crate::models::Priority::P4,
                    _ => crate::models::Priority::P3,
                };
                filter = filter.with_priority(prio);
            }

            // Add search filter
            if let Some(ref search_term) = search {
                filter = filter.with_search(search_term.clone());
            }

            // Build sort
            let sort = TaskSort::new(sort_field, sort_order);

            // Call the repository
            match commands::list_tasks_with_dyn(repository.as_ref(), filter, sort) {
                Ok(tasks) => {
                    // Apply limit (pagination)
                    let tasks: Vec<_> = tasks.into_iter().take(limit as usize).collect();

                    if tasks.is_empty() {
                        println!("No tasks found.");
                    } else {
                        // Print table header
                        println!(
                            "{:<38} | {:<30} | {:^8} | {:^12}",
                            "ID", "TITLE", "PRIORITY", "STATUS"
                        );
                        println!("{:-<38}-+-{:-<30 }-+-{:-<8 }-+-{:-<12}", "", "", "", "");

                        // Print each task
                        for task in tasks {
                            let title = if task.title.len() > 30 {
                                format!("{}...", &task.title[..27])
                            } else {
                                task.title
                            };
                            println!(
                                "{:<38} | {:<30} | {:^8} | {:^12}",
                                &task.id[..8],
                                title,
                                format!("{:?}", task.priority),
                                format!("{:?}", task.status)
                            );
                        }

                        println!("\nTotal: {} task(s)", limit);
                    }
                }
                Err(e) => {
                    eprintln!("Error listing tasks: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Show { id } => {
            match commands::get_task_with_dyn(repository.as_ref(), id.clone()) {
                Ok(task) => {
                    println!("Task Details:");
                    println!("  ID:          {}", task.id);
                    println!("  Title:       {}", task.title);
                    println!(
                        "  Description: {}",
                        task.description.as_deref().unwrap_or("N/A")
                    );
                    println!("  Priority:    {:?}", task.priority);
                    println!("  Status:      {:?}", task.status);
                    println!("  Created:     {}", task.created_at);
                    println!("  Updated:     {}", task.updated_at);
                    println!(
                        "  Due Date:    {}",
                        task.due_date
                            .map(|d| d.to_string())
                            .unwrap_or_else(|| "N/A".to_string())
                    );
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
