use std::{fs::File, io::{Read, Write}};

use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug,Serialize,Deserialize)]
pub struct DataOffset {
    pub file_id : String,
    pub slot : u64,
    pub index : u64,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct IndexEntry {
    pub pubkey : Pubkey,
    pub state : IndexState,
    pub data_offset : DataOffset
}

#[derive(Debug,Serialize,Deserialize)]
pub enum IndexState {
    Free,
    ZeroSlots,
    SingleElement,
    MultipleSlots
}

#[derive(Debug,Serialize,Deserialize)]
pub struct DataBucket {
    pub slots : Vec<DataOffset>
}

#[derive(Debug,Serialize,Deserialize)]
pub struct SingleBucket {
    pub index_bucket : IndexEntry,
    pub data_bucket : Vec<DataBucket>
}

// Currently implementing single_bucket mechanism only
#[derive(Debug,Serialize,Deserialize)]
pub struct Bucket {
    pub bucket : Vec<SingleBucket>
}

impl Default for Bucket {
    fn default() -> Self {
        Bucket {
            bucket : Vec::new()
        }
    }
}
impl Bucket {

    pub fn add_new_account_entry(
        &mut self,
        pubkey : Pubkey,
        slot  : u64,
        index : u64
    ) {
        let default_bucket_file_string = "accounts/default_bucket/bucket.txt";
        let file_id = "snapshots/slots/0/accounts/slot-0-block-1.txt".to_string();
        let new_single_entry_bucket_for_the_account = SingleBucket {
            index_bucket : IndexEntry {
                pubkey,
                state : IndexState::SingleElement,
                data_offset : DataOffset {
                    file_id,
                    index,
                    slot
                }
            },
            data_bucket : Vec::new()
        };

        self.bucket.push(new_single_entry_bucket_for_the_account);
        
        let new_serialize_content = bincode::serialize(&self.bucket).expect("serialization failed");
        let mut data_file = File::create(default_bucket_file_string).expect("creation failed");
        data_file.write(&new_serialize_content).expect("write failed");
    }

    pub fn read_accounts_from_file() -> Vec<SingleBucket> {
        let file_string = "accounts/default_bucket/bucket.txt";
        
        let mut data_file = File::open(&file_string).expect("file open failed");
    
        let mut buffer = Vec::new();
        data_file.read_to_end(&mut buffer).expect("read failed");
    
        if buffer.is_empty() {
            panic!("File is empty! Nothing to deserialize.");
        }
    
        let account_index_data: Vec<SingleBucket> = bincode::deserialize(&buffer)
            .expect("deserialization failed");
        
        account_index_data
    }



}

mod tests {
    use solana_sdk::{signature::Keypair, signer::Signer};

    use super::Bucket;

    #[test]
    fn test_add_new_account_index() {
        let mut bucket = Bucket::default(); 
        bucket.add_new_account_entry(Keypair::new().pubkey(), 0, 0);
    }

    #[test]
    fn test_read_from_account_file() {
        //let bucket = Bucket::default(); 
        let data =Bucket::read_accounts_from_file();
        println!("bucket_data {:?}",data)
    }
}