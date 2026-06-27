mod command;
mod display;
mod store;
mod task;

use clap::{Parser, Subcommand};

use crate::task::{Priority, Status};

#[derive(Parser)]
#[command(name = "tsk", about = "A simple task manger in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        title: String,
        #[arg(long, default_value = "medium")]
        priority: Priority,
        #[arg(long)]
        project: Option<String>,
    },
    List {
        #[arg(long)]
        project: Option<String>,
        #[arg(long)]
        priority: Option<Priority>,
        #[arg(long)]
        status: Option<Status>,
    },
    Edit {
        id: u32,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        priority: Option<Priority>,
        #[arg(long)]
        project: Option<String>,
    },
    Delete {
        id: u32,
    },
    Done {
        id: u32,
    },
    Search {
        keyword: String,
    },
    Summary,
}

fn main() {
    println!("Tsk-Task Manager\n");
}
