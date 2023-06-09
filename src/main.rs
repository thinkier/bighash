use std::path::Path;
use tokio::fs::{OpenOptions};
use crate::model::args::CliArgs;
use crate::model::merkle_tree::MerkleTree;
use crate::model::stream_processor::StreamProcessor;

pub mod model;

#[tokio::main]
async fn main() {
    let cli_args: CliArgs = argh::from_env();
    let print_live = !cli_args.hide_tree.unwrap_or(false);

    if cli_args.files.is_empty() {
        let proc = StreamProcessor::new(tokio::io::stdin());
        let tree = proc.digest("-", print_live).await.unwrap();
        print_tree(tree, "-").await;
    } else {
        for name in &cli_args.files {
            let path = Path::new(&name);

            if !path.is_file() {
                continue;
            }

            let file = OpenOptions::new().read(true).write(false).open(path).await.expect("Failed to open file");

            let proc = StreamProcessor::new(file);
            let tree = proc.digest(name, print_live).await.unwrap();

            print_tree(tree, &name).await;
        }
    }
}

async fn print_tree(tree: MerkleTree, name: &str) {
    let name = name.to_owned();
    tokio::task::spawn_blocking(move || {
        println!("{}\t{}", tree.merkle_hash(), name);
    });
}
