use core::panic;
use std::collections::HashMap;
use spl_associated_token_account::get_associated_token_address;
use spl_token::state::{Account as TokenAccount, Mint};
use solana_sdk::{program_pack::Pack, pubkey::Pubkey, signature::Keypair, signer::Signer};

#[derive(PartialEq, Eq, Clone,Default,Debug)]
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

#[derive(Debug)]
pub struct AccountsDB {
    pub key : HashMap<Pubkey,AccountSharedData>
}

impl Default for AccountsDB {
    fn default() -> Self {
        let accounts_db = HashMap::new();
        AccountsDB {
            key : accounts_db
        }
    }
}

impl AccountsDB {

    pub fn flush_new_account_into_db(&mut self,pubkey : Pubkey,account_shared_data : AccountSharedData) {

        if let Some(account) = self.key.get(&pubkey) {
            panic!("{:?} account alreaady exisits, cant create new",account)
        } else {
            self.key.insert(pubkey, account_shared_data);
        }
    }
    
    pub fn update_data(&mut self , new_data : Vec<u8> , account : Pubkey) {
        if let Some(account) = self.key.get_mut(&account) {
            account.data = new_data
        } else {
           panic!("Cant find the account in the AccountsDB")
        }
    }

   pub fn init_new_account_in_DB(
    &mut self,
    account : &Pubkey,
    space : usize,
    owner : &Pubkey
) -> AccountSharedData{
    let data = vec![0u8;space];
    let account_shared_data = AccountSharedData {
        lamports : 100_000_000,
        data,
        owner : *owner
    };
    self.flush_new_account_into_db(*account, account_shared_data.clone());
    account_shared_data
}

pub fn create_new_token_account (
    &mut self,
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
    let pubkey = get_associated_token_address(owner, mint);

    let account = self.init_new_account_in_DB(&pubkey, data.len(), &spl_token::id());


    // account.set_data_from_slice(&data);
    self.flush_new_account_into_db(pubkey,account.clone());
    self.update_data(data.to_vec(),pubkey);

    account
}

pub fn create_new_mint_account() {
    todo!()
}

pub fn update_accounts_db_after_transfer(&mut self,from : &Pubkey , to : &Pubkey , amount : u64) {
    {
        let from_account = self.key.get_mut(from).unwrap();
        from_account.lamports -= amount;
    }
    {
        let to_account = self.key.get_mut(to).unwrap();
        to_account.lamports += amount;
    }
}

pub fn get_account_data(&mut self,account : &Pubkey) -> &AccountSharedData {
let data = self.key.get(account).unwrap();
data
}

}

