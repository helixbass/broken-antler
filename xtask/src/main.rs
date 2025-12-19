use clap::{Parser, Subcommand};

mod add_ids;
mod concatenate_jsons;
mod create_sets_json;
mod seed;
mod shared;

use add_ids::add_ids;
use concatenate_jsons::concatenate_jsons;
use create_sets_json::create_sets_json;
use seed::seed;
pub use shared::{
    get_song_performances_by_set, json_seed_file_directory, json_seed_file_path, parse_json_file,
    workspace_root_directory, ShowOnlyOriginalId, SongPerformanceJson,
};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// seed the DB
    Seed,
    /// add UUID's to seed JSON file
    AddIds { file_name_root: String },
    /// concatenate multiple JSON files
    ConcatenateJsons { file_name_root: String },
    /// create sets seed file
    CreateSetsJson,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Seed => {
            seed().await?;
        }
        Command::AddIds { file_name_root } => {
            add_ids(&file_name_root).await?;
        }
        Command::ConcatenateJsons { file_name_root } => {
            concatenate_jsons(&file_name_root).await?;
        }
        Command::CreateSetsJson => {
            create_sets_json().await?;
        }
    }

    Ok(())
}
