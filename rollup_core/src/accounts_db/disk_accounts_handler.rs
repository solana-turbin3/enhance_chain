use std::{collections::HashMap, fs::File, io::{Read, Write}};
use bincode;

use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct AccountInFile {
    pub offset : u8, // will act as a index as_of_now
    pub data_len : usize,
    pub pubkey : Pubkey,
    pub lamports : u64,
    pub owner : Pubkey,
    pub executable : bool,
    pub data : Vec<u8>
    //rent_epoch : RentEpoch
}

pub struct AllAccountsInFile {
    pub accounts : Vec<AccountInFile>
}

impl Default for AllAccountsInFile {
    fn default() -> Self {
        Self {
            accounts : Vec::new()
        }
    }
}

impl AllAccountsInFile {

    pub fn add_new_acconunt(
        &mut self,
        slot : usize,
        account : AccountInFile
    ) {
        let file_string = format!("snapshots/slots/{}/accounts/{}.txt",slot,"slot-0-block-1");
        self.accounts.push(account.clone());
        let new_serialize_content = bincode::serialize(&self.accounts).expect("serialization failed");
        let mut data_file = File::create(file_string).expect("creation failed");
        data_file.write(&new_serialize_content).expect("write failed");
    }

    pub fn read_accounts_from_file(&self, slot: usize) -> Vec<AccountInFile> {
        let file_string = format!("snapshots/slots/{}/accounts/{}.txt", slot, "slot-0-block-1");
        let mut data_file = File::open(&file_string).expect("file open failed");
        let mut buffer = Vec::new();
        data_file.read_to_end(&mut buffer).expect("read failed");
        
        let accounts: Vec<AccountInFile> = bincode::deserialize(&buffer).expect("deserialization failed");
        accounts
    }

}


#[test]
fn test_basic_file_handling() {
    // Create a file
    let mut data_file = File::create("snapshots/slots/0/accounts/data.txt").expect("creation failed");

    // Write contents to the file
    data_file.write("Hello, World!".as_bytes()).expect("write failed");

    println!("Created a file data.txt");
}

#[test]
fn test_read_from_a_account_file() {
    let mut all_account = AllAccountsInFile::default();
    let data= all_account.read_accounts_from_file(0);
    println!("data {:?}",data)
}
