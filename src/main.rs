use std::path::Path;
use tokio::fs::{OpenOptions};
use crate::model::args::CliArgs;
use crate::model::merkle_tree::MerkleTree;
use crate::model::stream_processor::StreamProcessor;

pub mod model;

#[tokio::main]
async fn main() {
    let cli_args: CliArgs = argh::from_env();

    if cli_args.files.is_empty() {
        let proc = StreamProcessor::new(tokio::io::stdin());
        let tree = proc.digest(!cli_args.hide_tree).await.unwrap();
        print_tree(tree, "-", &cli_args).await;
    } else {
        for name in &cli_args.files {
            let file = OpenOptions::new().read(true).write(false).open(Path::new(&name)).await.expect("Failed to open file");
            let proc = StreamProcessor::new(file);
            let tree = proc.digest(!cli_args.hide_tree).await.unwrap();
            print_tree(tree, &name, &cli_args).await;
        }
    }
}

async fn print_tree(tree: MerkleTree, name: &str, cli_args: &CliArgs) {
    if !cli_args.hide_tree.unwrap_or(false) {
        for (i, hash) in tree.hashes().into_iter().enumerate() {
            println!("{}\t{}\t:{}", hash, name, i);
        }
    }

    let name = name.to_owned();
    tokio::task::spawn_blocking(move || {
        println!("{}\t{}", tree.merkle_hash(), name);
    });
}
