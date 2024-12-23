use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Manifest {
    // TODO: Fix
    // pub permissions: Permissions
}

#[derive(Clone, Subcommand)]
enum Command {
    Init {
        path: PathBuf,
    },
    Package {
        path: PathBuf,
        manifest_path: PathBuf,
        output_dir: PathBuf,
    }
}

#[derive(Clone, Parser)]
struct Arguments {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let args = Arguments::parse();
    match args.command {
        Command::Init { .. } => {}
        Command::Package { .. } => {}
    }
}
