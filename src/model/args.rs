#[derive(argh::FromArgs, Clone, Debug)]
#[argh(description = "merkle-tree based concurrent big file hasher")]
pub struct CliArgs {
    #[argh(switch)]
    /// do not print all leaf nodes in the merkle tree (print all hashes of the constituent blocks)
    pub hide_tree: Option<bool>,

    // #[argh(option)]
    // /// Skip the first n blocks
    // pub skip: Option<usize>,
    //
    // #[argh(option)]
    // /// Stop after hashing the prescribed amount of blocks
    // pub limit: Option<usize>,

    #[argh(positional, greedy)]
    /// the files to hash, if none are specified, an attempt will be made to read from stdin
    pub files: Vec<String>,
}
