//! TaskForge - CLI task management application
//!
//! Entry point for the TaskForge CLI tool.

use anyhow::Result;

mod cli;
mod commands;
mod config;
mod error;
mod filter;
mod models;
mod repository;
mod tag;
mod task;

fn main() -> Result<()> {
    println!("Hello, TaskForge!");
    Ok(())
}
