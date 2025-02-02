use solana_sdk::pubkey::Pubkey;

use super::user_handler::FullUserBase;

#[test]

fn test_register_app() {
    let mut full_user_base = FullUserBase::default();
    let program_id = Pubkey::new_unique();
    full_user_base.register_app(program_id);
    assert_eq!(full_user_base.user_base.len(),2);
    println!("{:?}",full_user_base)
}

#[test]
fn test_add_new_user_to_app() {
    let mut full_user_base = FullUserBase::default();
    let program_id = Pubkey::new_unique();
    full_user_base.register_app(program_id);
    assert_eq!(full_user_base.user_base.len(),2);

    let new_user1 = Pubkey::new_unique();
    let new_user2 = Pubkey::new_unique();
    full_user_base.add_new_user_to_app(program_id, new_user1);
    full_user_base.add_new_user_to_app(program_id, new_user2);
    assert_eq!(full_user_base.user_base.len(),2);
    println!("{:?}",full_user_base);
    
}

#[test]
fn update_user() {
    let mut full_user_base = FullUserBase::default();
    let program_id = Pubkey::new_unique();
    full_user_base.register_app(program_id);
    assert_eq!(full_user_base.user_base.len(),2);

    let new_user1 = Pubkey::new_unique();
    let new_user2 = Pubkey::new_unique();
    full_user_base.add_new_user_to_app(program_id, new_user1);
    full_user_base.add_new_user_to_app(program_id, new_user2);
    assert_eq!(full_user_base.user_base.len(),2);
    println!("{:?}",full_user_base);

    let user_for_update = Pubkey::new_unique();
    full_user_base.update_user(program_id, new_user2, user_for_update);
    println!("{:?} {:?}",new_user2,user_for_update);
    println!("{:?}",full_user_base);
}