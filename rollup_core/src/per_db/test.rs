use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

use crate::per_db::per_db::FullPerDB;

#[test]
fn test_add_new_transaction() {
    let mut full_per_db = FullPerDB {
        full_per_db: Vec::new(),
    };
    let app_program_id = Pubkey::new_unique();
    let signature = "test_signature_1".to_string();

    full_per_db.add(app_program_id, signature.clone());
    println!("Added signature: {:?}", signature);

    let signatures = full_per_db.get_signature_for_add(app_program_id);
    println!("Retrieved signatures: {:?}", signatures);
    assert_eq!(signatures, Some(vec![signature]));
}

#[test]
fn test_add_multiple_transactions_same_app() {
    let mut full_per_db = FullPerDB {
        full_per_db: Vec::new(),
    };
    let app_program_id = Pubkey::new_unique();
    let signature1 = "test_signature_1".to_string();
    let signature2 = "test_signature_2".to_string();

    full_per_db.add(app_program_id, signature1.clone());
    full_per_db.add(app_program_id, signature2.clone());
    println!("Added signatures: {:?}, {:?}", signature1, signature2);

    let signatures = full_per_db.get_signature_for_add(app_program_id);
    println!("Retrieved signatures: {:?}", signatures);
    assert_eq!(signatures, Some(vec![signature1, signature2]));
}

#[test]
fn test_add_multiple_transactions_different_apps() {
    let mut full_per_db = FullPerDB {
        full_per_db: Vec::new(),
    };
    let app_program_id1 = Pubkey::new_unique();
    let app_program_id2 = Pubkey::new_unique();
    let signature1 = "test_signature_1".to_string();
    let signature2 = "test_signature_2".to_string();

    full_per_db.add(app_program_id1, signature1.clone());
    full_per_db.add(app_program_id2, signature2.clone());
    println!(
        "Added signatures for different apps: {:?} -> {:?}, {:?} -> {:?}",
        app_program_id1, signature1, app_program_id2, signature2
    );

    let signatures1 = full_per_db.get_signature_for_add(app_program_id1);
    let signatures2 = full_per_db.get_signature_for_add(app_program_id2);
    println!(
        "Retrieved signatures: {:?} -> {:?}, {:?} -> {:?}",
        app_program_id1, signatures1, app_program_id2, signatures2
    );

    assert_eq!(signatures1, Some(vec![signature1]));
    assert_eq!(signatures2, Some(vec![signature2]));
}

#[test]
fn test_get_signature_for_nonexistent_app() {
    let mut full_per_db = FullPerDB {
        full_per_db: Vec::new(),
    };
    let app_program_id = Pubkey::new_unique();

    let signatures = full_per_db.get_signature_for_add(app_program_id);
    println!(
        "Retrieved signatures for nonexistent app: {:?} -> {:?}",
        app_program_id, signatures
    );
    assert_eq!(signatures, None);
}
