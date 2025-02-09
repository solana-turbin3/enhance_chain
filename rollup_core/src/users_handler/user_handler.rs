use core::panic;
use std::{collections::HashMap};

use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};

#[derive(Debug)]
pub struct Users {
    users : HashMap<String,Pubkey>
}

#[derive(Debug)]
pub struct AppUserBase {
    pub app_user_base : HashMap<Pubkey, Users>
}


impl Default for AppUserBase {
    fn default() -> Self {
        // app_user_base.app_user_base.insert(Pubkey::new_unique(), Users {users : vec![Pubkey::new_unique()]});
        AppUserBase {
            app_user_base : HashMap::new()
        }
    }
}

impl AppUserBase {
    // IMP 
    // into_iter() -> hashmap imo
    // find and find_map
    //Similar to find(), but instead of returning a reference to the item, it applies a function (map) to transform the found item before returning.

    pub fn register_app(&mut self, program_id: Pubkey) {
        if self.app_user_base.contains_key(&program_id) {
            panic!("App already exists");
        } else {
            self.app_user_base.insert(program_id, Users { users: HashMap::new() });
        }
    }

    pub fn add_new_user_to_app(&mut self, app_program_id:Pubkey) {
        // TODO:
        // handle case when user already exisits
        if self.app_user_base.contains_key(&app_program_id) {
            let users_vec = self.app_user_base.get_mut(&app_program_id).unwrap();
            let new_keypair = Keypair::new();
            let mut base_user_name = "user".to_string().to_owned();
            let last_new_name = (users_vec.users.len() +1).to_string();
            base_user_name.push_str(&last_new_name);
            users_vec.users.insert(base_user_name , new_keypair.pubkey());
        }  else {
            panic!("cant find the app")
        }
    }

    pub fn get_current_len_of_userbase_of_app(&mut self , app_program_id : Pubkey) -> usize {
        if self.app_user_base.contains_key(&app_program_id) {
            let uses_vec = self.app_user_base.get(&app_program_id).unwrap();
            let users_len = uses_vec.users.len();
            users_len
        } else {
            panic!("cant find the app")
        }
    }


}

