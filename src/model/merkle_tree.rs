use std::ops::{Index, IndexMut};
use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub enum MerkleTree {
    Leaf { hash: String },
    Node { left: Box<MerkleTree>, right: Box<MerkleTree> },
}

impl MerkleTree {
    /// Create an unbalanced (left-aligned) merkle tree with the specified number of leaf nodes.
    pub fn with_size(leafs: usize) -> MerkleTree {
        if leafs == 1 {
            MerkleTree::Leaf { hash: String::new() }
        } else {
            let left = Box::new(MerkleTree::with_size(leafs / 2));
            let right = Box::new(MerkleTree::with_size(leafs - (leafs / 2)));
            MerkleTree::Node { left, right }
        }
    }

    /// Count the number of leaf nodes in the tree.
    pub fn count(&self) -> usize {
        match self {
            MerkleTree::Leaf { .. } => 1,
            MerkleTree::Node { left, right, .. } => left.count() + right.count(),
        }
    }

    /// Retrieve (leaf) or calculate (node) the hash
    pub fn merkle_hash(&self) -> String {
        match self {
            MerkleTree::Leaf { hash } => hash.to_owned(),
            MerkleTree::Node { left, right } => {
                let left_hash = left.merkle_hash();
                let right_hash = right.merkle_hash();

                let mut hasher = Sha256::new();

                hasher.input_str(&left_hash);
                hasher.input_str(&right_hash);

                hasher.result_str()
            }
        }
    }

    /// Retrieve all constituent hashes in order
    pub fn hashes(&self) -> Vec<String> {
        match self {
            MerkleTree::Leaf { hash } => vec![hash.clone()],
            MerkleTree::Node { left, right } => {
                let mut hashes = left.hashes();
                hashes.extend(right.hashes());
                hashes
            }
        }
    }
}

impl Index<usize> for MerkleTree {
    type Output = MerkleTree;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            MerkleTree::Leaf { .. } => {
                if index > 0 {
                    panic!("Index out of bounds");
                }
                &self
            }
            MerkleTree::Node { left, right, .. } => {
                if index < left.count() {
                    &left[index]
                } else {
                    &right[index - left.count()]
                }
            }
        }
    }
}

impl IndexMut<usize> for MerkleTree {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            MerkleTree::Leaf { .. } => {
                if index > 0 {
                    panic!("Index out of bounds");
                }
                self
            }
            MerkleTree::Node { left, right, .. } => {
                if index < left.count() {
                    &mut left[index]
                } else {
                    &mut right[index - left.count()]
                }
            }
        }
    }
}

impl From<Vec<String>> for MerkleTree {
    fn from(hashes: Vec<String>) -> Self {
        let mut tree = MerkleTree::with_size(hashes.len());

        for (index, hash) in hashes.into_iter().enumerate() {
            tree[index] = MerkleTree::Leaf { hash };
        }

        return tree;
    }
}
