use std::mem;
use tokio::io::{BufReader, AsyncRead, AsyncReadExt};
use tokio::task::JoinHandle;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crate::model::merkle_tree::MerkleTree;

// 16MiB
pub const BLOCK_SIZE: usize = 1 << 24;
// 512MiB per file task
pub const CONCURRENCY_LIMIT: usize = 32;

pub struct StreamProcessor<T> {
    stream: T,
}

impl<T: AsyncRead + Unpin> StreamProcessor<T> {
    pub fn new(stream: T) -> StreamProcessor<T> {
        StreamProcessor {
            stream
        }
    }

    pub async fn digest(mut self, name: &str, print_live: bool) -> tokio::io::Result<MerkleTree> {
        let mut hashes = vec![];
        let mut handles: Vec<JoinHandle<String>> = Vec::with_capacity(CONCURRENCY_LIMIT);
        let mut read = BufReader::new(&mut self.stream);

        let mut i = 0;
        'stream: loop {
            // Read persistence system
            let mut cur = 0;
            let mut buf = vec![0u8; BLOCK_SIZE];

            // Read (up to) a block
            'block: loop {
                // Backpressure protection system turned logger-printer
                if handles.len() >= CONCURRENCY_LIMIT {
                    let handle = handles.remove(0);
                    let hash = handle.await.unwrap();

                    if print_live {
                        println!("{}\t{}\t:{}", hash, name, i);
                    }

                    hashes.push(hash);
                    i += 1;
                }

                let n = read.read(&mut buf[cur..BLOCK_SIZE]).await?;

                if n == 0 && cur == 0 {
                    break 'stream;
                } else if cur + n < BLOCK_SIZE && n > 0 {
                    cur += n;
                    continue 'block;
                }

                // Hash the block in a blocking task to prevent async workers from getting exhausted
                {
                    let buf = mem::replace(&mut buf, vec![0u8; BLOCK_SIZE]);
                    let len = cur + n;
                    handles.push(tokio::task::spawn_blocking(move || {
                        let mut hasher = Sha256::new();
                        hasher.input(&buf[0..len]);

                        let hash = hasher.result_str();

                        hash
                    }));
                }
                cur = 0;
            }
        }

        for handle in handles {
            let hash = handle.await.unwrap();

            if print_live {
                println!("{}\t{}\t:{}", hash, name, i);
            }

            hashes.push(hash);
            i += 1;
        }

        Ok(MerkleTree::from(hashes))
    }
}
