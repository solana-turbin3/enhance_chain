use solana_sdk::{program_pack::Pack, signature::Keypair, signer::Signer};
use spl_associated_token_account::get_associated_token_address;
use spl_token::state::{Account as TokenAccount, Mint};
use std::io::Cursor;
use tokio::io::AsyncReadExt;

use super::ram_accounts_handler::AccountsDB;

#[test]

pub fn test_new_account_init() {
    let mut account_db = AccountsDB::default();
    let owner = Keypair::new().pubkey();
    let test_account = Keypair::new().pubkey();

    let account_size = 32;

    account_db.init_new_account_in_DB(&test_account, account_size, &owner);

    println!("{:?}", account_db);
}

#[test]
pub fn create_new_token_account() {
    let mut account_db = AccountsDB::default();
    let owner = Keypair::new().pubkey();
    let mint = Keypair::new().pubkey();

    let account = get_associated_token_address(&owner, &mint);

    account_db.create_new_token_account(&owner, &mint, 10);

    let new_account_data = account_db.get_account_data(&account);

    let new_account_data = TokenAccount::unpack(&new_account_data.data).unwrap();
    println!("unpack{:?}", new_account_data);

    println!("account_db {:?}", account_db)
}
