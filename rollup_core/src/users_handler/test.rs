use solana_sdk::pubkey::Pubkey;

use crate::users_handler::user_handler::AppUserBase;


#[test]

fn test_register_app() {
    let mut app_user_base = AppUserBase::default();
    let program_id = Pubkey::new_unique();
    app_user_base.register_app(program_id);
    assert_eq!(app_user_base.app_user_base.len(),2);
    println!("{:?}",app_user_base)
}

#[test]
fn test_add_new_user_to_app() {
    let mut app_user_base = AppUserBase::default();
    let program_id = Pubkey::new_unique();
    app_user_base.register_app(program_id);
    assert_eq!(app_user_base.app_user_base.len(),1);

    app_user_base.add_new_user_to_app(program_id);
    app_user_base.add_new_user_to_app(program_id);
    assert_eq!(app_user_base.get_current_len_of_userbase_of_app(program_id), 2);
    println!("{:?}",app_user_base);
}

#[test]

fn get_user_key() {
    let mut app_user_base = AppUserBase::default();
    let program_id = Pubkey::new_unique();
    app_user_base.register_app(program_id);
    assert_eq!(app_user_base.app_user_base.len(),1);

    app_user_base.add_new_user_to_app(program_id);

    //
    let new_added_keypair = app_user_base.get_keypair_from_user_name(program_id, "user1".to_string());
    println!("{:?}",new_added_keypair)    
    
}