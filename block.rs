use std::timeSystemTime;
use crypto_hash::{hex_digest, Algorithm};

#[derive(Debug)]
struct Block{
    index: u32,
    timestamp: u64,
    data: String,
    pervious_hash: String,
    hash: String,
}