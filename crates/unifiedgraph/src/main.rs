mod cli;

use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Extract(args) => {
            println!("Extract command with paths: {:?}", args.paths);
            println!("Output: {:?}", args.output);
            println!("With edges: {:?}", args.with_edges);
        }
        Commands::Query(args) => {
            println!("Query command with pattern: {}", args.pattern);
            println!("Node type filter: {:?}", args.node_type);
            println!("JSON output: {:?}", args.json);
        }
        Commands::Serve(args) => {
            println!("Serve command on {}:{} with CORS: {:?}", args.host, args.port, args.cors);
        }
        Commands::Export(args) => {
            println!("Export command: format={:?}, output={:?}", args.format, args.output);
            println!("Obsidian export: {:?}", args.obsidian);
            println!("Wiki export: {:?}", args.wiki);
        }
        Commands::Install(args) => {
            println!("Install command with hook type: {}", args.hook_type);
        }
        _ => {
            println!("Command not yet implemented");
        }
    }

    Ok(())
}
