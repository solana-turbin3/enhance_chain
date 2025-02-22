use std::collections::HashMap;
use spl_associated_token_account::get_associated_token_address;
use spl_token::state::{Account as TokenAccount, Mint};
use solana_sdk::{program_pack::Pack, pubkey::Pubkey, signature::Keypair, signer::Signer};

#[derive(PartialEq, Eq, Clone, Default)]
pub struct AccountSharedData {
    /// lamports in the account
    lamports: u64,
    /// data held in this account
    data: Vec<u8>,
    /// the program that owns this account. If executable, the program that loads this account.
    owner: Pubkey,
    // this account's data contains a loaded program (and is now read-only)
    //executable: bool,
    // the epoch at which this account will next owe rent
    //rent_epoch: Epoch,

    //size
}

pub struct AccountsDB {
    pub key : HashMap<Pubkey,AccountSharedData>
}

impl AccountsDB {

    pub fn flush_new_account_into_db(&mut self,pubkey : Pubkey,account_shared_data : AccountSharedData) {

        if let Some(account) = self.key.get(&pubkey) {
            // TODO:
            // error 
            //account already exists
        } else {
            self.key.insert(pubkey, account_shared_data);
        }
    }
    pub fn update_data(&mut self , new_data : Vec<u8> , account : Pubkey) {
        //self.data = new_data
        if let Some(account) = self.key.get_mut(&account) {
            account.data = new_data
        } else {
            //TODO:
            // account doesnt exists in the accounts DB
        }
    }
}

impl AccountSharedData {

    pub fn new_with_no_data(
        &mut self,
        lamports : u64,
        space : usize,
        owner : &Pubkey
    ) -> Self {
        let data = vec![0u8;space];
        AccountSharedData {
            lamports,
            data,
            owner : *owner
        }
    }


    pub fn create_new_token_account (
        &mut self,
        account_db : &mut AccountsDB,
        owner: &Pubkey,
        mint : &Pubkey,
        amount : u64
    ) -> AccountSharedData {
        let data = {
            let mut data = [0; TokenAccount::LEN];
            TokenAccount::pack(
                TokenAccount {
                    mint: *mint,
                    owner: *owner,
                    amount,
                    state: spl_token::state::AccountState::Initialized,
                    ..Default::default()
                },
                &mut data,
            )
            .unwrap();
            data
        };

        let account = self.new_with_no_data(100_000_000, data.len(), &spl_token::id());

        let pubkey = get_associated_token_address(owner, mint);

        // account.set_data_from_slice(&data);
        account_db.update_data(data.to_vec(),pubkey);
        account_db.flush_new_account_into_db(pubkey,account.clone());

        account
    }
    
}

