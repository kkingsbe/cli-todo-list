#![allow(dead_code)]
//! TaskForge - CLI task management application
//!
//! Entry point for the TaskForge CLI tool.

use anyhow::Result;
use clap::Parser;
use std::env;
use std::str::FromStr;
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

use crate::cli::{Cli, Commands, OutputFormat};
use crate::commands::complete_task_with_dyn;
use crate::commands::create_task_with_dyn;
use crate::commands::delete_task_with_dyn;
use crate::commands::reopen_task_with_dyn;
use crate::commands::update_task_with_dyn;
use crate::config::load_config;
use crate::error::AppError;
use crate::filter::{SortOrder, TagFilter, TaskFilter, TaskSort, TaskSortField};
use crate::models::{Priority, Status, Tag};
use crate::repository::{Repository, RepositoryError, SqliteRepository};

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
                    // Handle tags if provided
                    if let Some(tags) = tag {
                        for tag_name in tags {
                            // Try to get existing tag, or create new one
                            let tag = match repository.get_tag_by_name(&tag_name) {
                                Ok(existing_tag) => existing_tag,
                                Err(RepositoryError::NotFound(_)) => {
                                    // Create new tag
                                    let new_tag = Tag::new(tag_name);
                                    repository.create_tag(&new_tag).map_err(|e| {
                                        anyhow::anyhow!("Failed to create tag: {}", e)
                                    })?;
                                    new_tag
                                }
                                Err(e) => {
                                    anyhow::bail!("Error looking up tag: {}", e);
                                }
                            };

                            // Link tag to task
                            repository.add_tag_to_task(&task.id, &tag.id).map_err(|e| {
                                anyhow::anyhow!("Failed to link tag to task: {}", e)
                            })?;
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
            format,
            due_before,
            due_after,
            due,
            ..
        } => {
            // Build filter from command arguments
            let mut filter = TaskFilter::new();

            // Parse status filter
            if let Some(ref status_str) = status {
                match status_str.to_lowercase().as_str() {
                    "completed" | "complete" => {
                        filter = filter.with_status(Status::Completed);
                    }
                    "incomplete" | "pending" => {
                        filter = filter.with_status(Status::Incomplete);
                    }
                    _ => {
                        eprintln!("Warning: Invalid status '{}', ignoring", status_str);
                    }
                }
            }

            // Parse priority filter
            if let Some(p) = priority {
                if let Ok(priority_enum) = crate::models::Priority::from_str(&format!("P{}", p)) {
                    filter = filter.with_priority(priority_enum);
                } else {
                    eprintln!("Warning: Invalid priority {}, ignoring", p);
                }
            }

            // Add search filter
            if let Some(ref search_term) = search {
                filter = filter.with_search(search_term.clone());
            }

            // Parse due_before date filter
            if let Some(ref due_before_str) = due_before {
                if let Ok(date) = chrono::NaiveDate::parse_from_str(due_before_str, "%Y-%m-%d") {
                    let datetime = date.and_hms_opt(23, 59, 59).unwrap().and_utc();
                    filter.due_before = Some(datetime);
                } else {
                    eprintln!(
                        "Warning: Invalid due_before date '{}', ignoring (expected YYYY-MM-DD)",
                        due_before_str
                    );
                }
            }

            // Parse due_after date filter
            if let Some(ref due_after_str) = due_after {
                if let Ok(date) = chrono::NaiveDate::parse_from_str(due_after_str, "%Y-%m-%d") {
                    // Add 1 day so --due-after 2026-03-01 means after end of that day
                    let datetime = (date + chrono::Duration::days(1))
                        .and_hms_opt(0, 0, 0)
                        .unwrap()
                        .and_utc();
                    filter.due_after = Some(datetime);
                } else {
                    eprintln!(
                        "Warning: Invalid due_after date '{}', ignoring (expected YYYY-MM-DD)",
                        due_after_str
                    );
                }
            }

            // Parse due (exact date) filter
            if let Some(ref due_str) = due {
                if let Ok(date) = chrono::NaiveDate::parse_from_str(due_str, "%Y-%m-%d") {
                    let datetime = date.and_hms_opt(0, 0, 0).unwrap().and_utc();
                    filter.due = Some(datetime);
                } else {
                    eprintln!(
                        "Warning: Invalid due date '{}', ignoring (expected YYYY-MM-DD)",
                        due_str
                    );
                }
            }

            // Build sort from command arguments
            let sort_field = match sort_by.to_lowercase().as_str() {
                "priority" => TaskSortField::Priority,
                "due" | "duedate" => TaskSortField::DueDate,
                "title" => TaskSortField::Title,
                "updated" | "updatedat" => TaskSortField::UpdatedAt,
                _ => TaskSortField::CreatedAt, // default to created
            };

            let sort_order = match order.to_lowercase().as_str() {
                "asc" | "ascending" => SortOrder::Ascending,
                _ => SortOrder::Descending, // default to descending
            };

            let sort = TaskSort::new(sort_field, sort_order);

            // Fetch tasks from repository
            match repository.list_tasks(&filter, &sort) {
                Ok(tasks) => {
                    match format {
                        OutputFormat::Table => {
                            // Table format output
                            if tasks.is_empty() {
                                println!("No tasks found.");
                            } else {
                                // Print table header
                                println!(
                                    "{:<36} | {:<30} | {:<8} | {:<12}",
                                    "ID", "Title", "Priority", "Status"
                                );
                                println!("{:-<36}-+-{:-<30}-+-{:-<8}-+-{:-<12}", "", "", "", "");
                                // Print each task
                                for task in &tasks {
                                    println!(
                                        "{:<36} | {:<30} | {:<8} | {:<12}",
                                        task.id,
                                        if task.title.len() > 30 {
                                            &task.title[..27]
                                        } else {
                                            &task.title
                                        },
                                        format!("{:?}", task.priority),
                                        format!("{:?}", task.status)
                                    );
                                }
                            }
                        }
                        OutputFormat::Plain => {
                            // Plain text format
                            if tasks.is_empty() {
                                println!("No tasks found.");
                            } else {
                                for task in &tasks {
                                    println!(
                                        "ID: {}, Title: {}, Priority: {:?}, Status: {:?}",
                                        task.id, task.title, task.priority, task.status
                                    );
                                    if let Some(ref desc) = task.description {
                                        println!("  Description: {}", desc);
                                    }
                                    if let Some(due) = task.due_date {
                                        println!("  Due Date: {}", due.format("%Y-%m-%d"));
                                    }
                                    println!();
                                }
                            }
                        }
                        OutputFormat::Json => {
                            // JSON format output
                            if tasks.is_empty() {
                                println!("[]");
                            } else {
                                let json_output =
                                    serde_json::to_string_pretty(&tasks).map_err(|e| {
                                        anyhow::anyhow!("Failed to serialize tasks to JSON: {}", e)
                                    })?;
                                println!("{}", json_output);
                            }
                        }
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
        Commands::Edit {
            id,
            title,
            description,
            priority,
            status,
            due,
            add_tag,
            remove_tag,
        } => {
            match update_task_with_dyn(
                repository.as_ref(),
                id.clone(),
                title,
                description,
                priority,
                status,
                due,
            ) {
                Ok(task) => {
                    println!("Updated task: {}", task.id);
                    println!("  Title:       {}", task.title);
                    println!(
                        "  Description: {}",
                        task.description.as_deref().unwrap_or("N/A")
                    );
                    println!("  Priority:    {:?}", task.priority);
                    println!("  Status:      {:?}", task.status);
                    println!(
                        "  Due Date:    {}",
                        task.due_date
                            .map(|d| d.to_string())
                            .unwrap_or_else(|| "N/A".to_string())
                    );

                    // Handle tag additions
                    if let Some(tag_name) = add_tag {
                        match repository.get_tag_by_name(&tag_name) {
                            Ok(tag) => {
                                // Tag exists, link to task
                                if let Err(e) = repository.add_tag_to_task(&id, &tag.id) {
                                    eprintln!("Error adding tag: {}", e);
                                } else {
                                    println!("  Added tag: {}", tag_name);
                                }
                            }
                            Err(RepositoryError::NotFound(_)) => {
                                // Tag doesn't exist, create it
                                let new_tag = Tag::new(tag_name.clone());
                                match repository.create_tag(&new_tag) {
                                    Ok(_) => {
                                        // Now look up the tag to get its ID
                                        match repository.get_tag_by_name(&tag_name) {
                                            Ok(tag) => {
                                                if let Err(e) =
                                                    repository.add_tag_to_task(&id, &tag.id)
                                                {
                                                    eprintln!("Error adding tag: {}", e);
                                                } else {
                                                    println!("  Added tag: {}", tag_name);
                                                }
                                            }
                                            Err(e) => {
                                                eprintln!("Error looking up created tag: {}", e);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Error creating tag: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Error looking up tag: {}", e);
                            }
                        }
                    }

                    // Handle tag removals
                    if let Some(tag_name) = remove_tag {
                        match repository.get_tag_by_name(&tag_name) {
                            Ok(tag) => {
                                // Remove tag from task
                                if let Err(e) = repository.remove_tag_from_task(&id, &tag.id) {
                                    eprintln!("Error removing tag: {}", e);
                                } else {
                                    println!("  Removed tag: {}", tag_name);
                                }
                            }
                            Err(e) => {
                                eprintln!("Error finding tag: {}", e);
                            }
                        }
                    }
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
        Commands::Delete { id, force } => match delete_task_with_dyn(repository.as_ref(), id.clone(), force) {
            Ok(()) => {
                println!("Deleted task: {}", id);
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
        Commands::Complete { id } => match complete_task_with_dyn(repository.as_ref(), id) {
            Ok(task) => {
                println!("Completed task: {}", task.id);
                println!("  Title:  {}", task.title);
                println!("  Status: {:?}", task.status);
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
        },
        Commands::Reopen { id } => match reopen_task_with_dyn(repository.as_ref(), id) {
            Ok(task) => {
                println!("Reopened task: {}", task.id);
                println!("  Title:  {}", task.title);
                println!("  Status: {:?}", task.status);
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
        },
        Commands::Tag { .. } => {
            // TODO: Implement tag command
            tracing::info!("Tag command not yet implemented");
        }
        Commands::Tags => {
            match repository.list_tags(&TagFilter::default()) {
                Ok(tags) => {
                    if tags.is_empty() {
                        println!("No tags found.");
                    } else {
                        // Print table header
                        println!("{:<20} | {:<15}", "Tag Name", "Usage Count");
                        println!("{:-<20}-+-{:-<15}", "", "");
                        // Print each tag with its usage count
                        for tag_with_count in &tags {
                            println!(
                                "{:<20} | {:<15}",
                                tag_with_count.tag.name, tag_with_count.usage_count
                            );
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error listing tags: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
