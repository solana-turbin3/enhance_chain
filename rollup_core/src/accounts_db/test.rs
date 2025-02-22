use solana_sdk::{signature::Keypair, signer::Signer};

use super::accounts_db::AccountsDB;

#[test]

pub fn test_new_account_init() {
    let mut account_db = AccountsDB::default();
    let owner = Keypair::new().pubkey();
    let test_account = Keypair::new().pubkey();


     let account_size = 32;

     account_db.init_new_account_in_DB(&test_account, account_size, &owner);

     println!("{:?}",account_db);
}