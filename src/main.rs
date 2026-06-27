mod command;
mod display;
mod store;
mod task;

use clap::{Parser, Subcommand};

use crate::{
    store::{TaskStore, get_storage_path},
    task::{Priority, Status},
};

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
        project: Option<Option<String>>,
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

    let path = get_storage_path();
    let mut store = TaskStore::load(&path).expect("Failed to load tasks.");

    let cli = Cli::parse();

    let output = match cli.command {
        Commands::Add {
            title,
            priority,
            project,
        } => command::handle_add(&mut store, title, priority, project),
        Commands::List {
            project,
            priority,
            status,
        } => command::handle_list(&store, project, priority, status),
        Commands::Edit {
            id,
            title,
            priority,
            project,
        } => match command::handle_edit(&mut store, id, title, priority, project) {
            Ok(msg) => msg,
            Err(e) => e,
        },
        Commands::Delete { id } => match command::handle_delete(&mut store, id) {
            Ok(msg) => msg,
            Err(e) => e,
        },
        Commands::Done { id } => match command::handle_mark_done(&mut store, id) {
            Ok(msg) => msg,
            Err(e) => e,
        },
        Commands::Search { keyword } => command::handle_search(&store, keyword.as_str()),
        Commands::Summary => command::handle_summary(&store),
    };

    println!("{}", output);

    store.save(&path).expect("Failed to save tasks");
}
