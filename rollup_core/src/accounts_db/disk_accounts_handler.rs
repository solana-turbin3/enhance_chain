use std::{collections::HashMap, fs::File, io::Write};
use bincode;

use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug,Serialize,Deserialize)]
pub struct AccountInFile {
   // pub offset : u8,
    pub data_len : usize,
    pub pubkey : Pubkey,
    pub lamports : u64,
    pub owner : Pubkey,
    pub executable : bool,
    pub data : Vec<u8>
    //rent_epoch : RentEpoch
}

// pub struct AllAccountsInFile {
//     // 1.Offset , 2.Data
//     pub accounts : Vec<AccountInFile>
// }

impl AccountInFile {
    pub fn add_new_account(
        slot : usize,
        pubkey : Pubkey,
        account : AccountInFile
    ) {
        let file_string = format!("snapshots/slots/{}/accounts/{}.txt",slot,pubkey);
        let serialized_account = bincode::serialize(&account).expect("serialization failed");
        let mut data_file = File::create(file_string).expect("creation failed");
        data_file.write(&serialized_account).expect("write failed");
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