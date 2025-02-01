use std::collections::HashMap;

use solana_sdk::pubkey::Pubkey;

pub struct Users {
    users : Vec<Pubkey>
}

pub struct AppUserBase {
    app_user_base : HashMap<Pubkey, Users>
}

pub struct FullUserBase {
    user_base : Vec<AppUserBase>
}

impl FullUserBase {
    // find and find_map
    //Returns an immutable or mutable reference to the item that matches the predicate.
    //Similar to find(), but instead of returning a reference to the item, it applies a function (map) to transform the found item before returning.
    
    pub fn register_app(&mut self, program_id:Pubkey) {
        if let Some(user_base) = self.user_base.iter_mut()
        .find(|app_user_base| app_user_base.app_user_base.contains_key(&program_id)) {
            todo!()
        } else {
            let mut new_app_user_base = AppUserBase {
                app_user_base : HashMap::new()
            };
            new_app_user_base.app_user_base.insert(program_id, {
                Users {
                    users : vec![]
                }
            });
            self.user_base.push(new_app_user_base);
        }
    }

    pub fn add_new_user_to_app(&mut self, app_program_id:Pubkey,new_user:Pubkey) {
        if let Some(user_base) = self.user_base
        .iter_mut()
        .find_map(|data| data.app_user_base.get_mut(&app_program_id)) {
            user_base.users.push(new_user);
        } else  {
            self.register_app(app_program_id);
        }
    }

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

