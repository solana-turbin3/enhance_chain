use core::panic;
use std::{collections::HashMap};

use solana_sdk::pubkey::Pubkey;

#[derive(Debug)]
pub struct Users {
    users : Vec<Pubkey>
}

#[derive(Debug)]
pub struct AppUserBase {
    pub app_user_base : HashMap<Pubkey, Users>
}

#[derive(Debug)]
pub struct FullUserBase {
    pub user_base : Vec<AppUserBase>
}

impl Default for FullUserBase {
    fn default() -> Self {
        let mut app_user_base = AppUserBase {
            app_user_base : HashMap::new()
        };
        app_user_base.app_user_base.insert(Pubkey::new_unique(), Users {users : vec![Pubkey::new_unique()]});
        FullUserBase {
            user_base : vec![app_user_base]
        }
    }
}

impl FullUserBase {
    // IMP 
    // into_iter() -> hashmap imo
    // find and find_map
    //Similar to find(), but instead of returning a reference to the item, it applies a function (map) to transform the found item before returning.
    pub fn register_app(&mut self, program_id:Pubkey) {
        if let Some(user_base) = self.user_base.iter_mut()
        .find_map(|app_user_base| app_user_base.app_user_base.get_mut(&program_id)) {
            panic!("App already exists")
        } else {
            self.user_base.push(AppUserBase {
                app_user_base: vec![(program_id, Users { users: vec![] })].into_iter().collect(),
            });
        }
    }

    pub fn add_new_user_to_app(&mut self, app_program_id:Pubkey,new_user:Pubkey) {
        // TODO:
        // handle case when user already exisits
        if let Some(user_base) = self.user_base
        .iter_mut()
        .find_map(|data| data.app_user_base.get_mut(&app_program_id)) {
            user_base.users.push(new_user);
        } else  {
            panic!("cant find the app")
        }
    }

    //IMP - position
    pub fn update_user(
        &mut self,
        program_id : Pubkey,
        old_user_id : Pubkey,
        new_user_id : Pubkey
    ) {
        if let Some(app_base) = self.user_base.iter_mut().find_map(|base| base.app_user_base.get_mut(&program_id)) {
            if let Some(index) = app_base.users.iter().position(|&u| u==old_user_id) {
                app_base.users[index] = new_user_id
            }
        }
    }
}

