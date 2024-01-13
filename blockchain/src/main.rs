use std::time::SystemTime;
use crypto_hash::{hex_digest, Algorithm};

// Define the structure for a block
#[derive(Debug)]
struct Block {
   index: u32,
   timestamp: u64,
   data: String,
   previous_hash: String,
   hash: String,
}

// Function to calculate the SHA256 hash of a block
fn calculate_hash(index: u32, timestamp: u64, data: &str, previous_hash: &str) -> String {
   let input = format!("{}{}{}{}", index, timestamp, data, previous_hash);
   hex_digest(Algorithm::SHA256, input.as_bytes())
}

// Function to create a new block
fn create_block(index: u32, data: &str, previous_block: &Block) -> Block {
   let timestamp = SystemTime::now()
   .duration_since(SystemTime::UNIX_EPOCH)
   .expect("Time went backwards")
   .as_secs();
  let previous_hash = previous_block.hash.clone();
  let hash = calculate_hash(index, timestamp, data, &previous_hash);
  Block {
     index,
     timestamp,
     data: data.to_string(),
     previous_hash,
     hash,
   }
}

// Function to validate a block's integrity
fn is_block_valid(block: &Block, previous_block: &Block) -> bool {
   block.index == previous_block.index + 1 &&
   block.previous_hash == previous_block.hash &&
   block.hash == calculate_hash(block.index, block.timestamp, &block.data, &block.previous_hash)
}

fn main() {
   // Create the genesis block
   let genesis_block = Block {
   index: 0,
   timestamp: SystemTime::now()
   .duration_since(SystemTime::UNIX_EPOCH)
   .expect("Time went backwards")
   .as_secs(),
   data: String::from("Genesis Block"),
   previous_hash: String::from("0"),
   hash: String::new(), // Placeholder, will be calculated later
 };

// Create subsequent blocks
 let block1 = create_block(1, "Transaction Data 1", &genesis_block);
 let block2 = create_block(2, "Transaction Data 2", &block1);

// Validate blocks
 let is_block1_valid = is_block_valid(&block1, &genesis_block);
 let is_block2_valid = is_block_valid(&block2, &block1);

// Print blocks and validation results
 println!("{:?}", genesis_block);
 println!("{:?}", block1);
 println!("{:?}", block2);
 println!("Block 1 is valid: {}", is_block1_valid);
 println!("Block 2 is valid: {}", is_block2_valid);
}