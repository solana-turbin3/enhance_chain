use solana_sdk::pubkey::Pubkey;

use super::read_write_locks::ThreadAwareLocks;

/// LOCKING TESTS ///

//TODO
//Some(ThreadSet::only(2))


// appply multiple read_locks 
#[test]
fn test_account_read_locks() {
    const TEST_NUM_THREADS: usize = 4;
    const SELECTED_THREAD: usize= 1;
    let mut locks = ThreadAwareLocks::new(TEST_NUM_THREADS);
    let pk1 = Pubkey::new_unique();
    locks.read_account_lock(pk1, 1);
    locks.read_account_lock(pk1, 1);

    assert_eq!(
        locks.locks.get(&pk1).unwrap().read_lock.as_ref().unwrap().lock_count[SELECTED_THREAD],
        2
    );
    println!("{:?}",locks);
}

// apply single write_lock
#[test]
fn test_account_write_lock() {
    const TEST_NUM_THREADS: usize = 4;
    let mut locks = ThreadAwareLocks::new(TEST_NUM_THREADS);

    let pk1 = Pubkey::new_unique();
    locks.write_lock_account(pk1, 1);
    println!("{:?}",locks);

    assert_eq!(
        locks.locks.get(&pk1).unwrap().write_lock.as_ref().unwrap().lock_count,
        1
    )
}

// apply multiple write_locks
#[test]
fn test_account_write_lock_multiple() {
    const TEST_NUM_THREADS: usize = 4;
    let mut locks = ThreadAwareLocks::new(TEST_NUM_THREADS);

    let pk1 = Pubkey::new_unique();
    locks.write_lock_account(pk1, 1);
    locks.write_lock_account(pk1, 1);
    println!("{:?}",locks);

    assert_eq!(
        locks.locks.get(&pk1).unwrap().write_lock.as_ref().unwrap().lock_count,
        2
    )
}

// outstanding read lock must be on same thread
#[test]
#[should_panic(expected="outstanding read lock must be on same thread")]
fn test_read_write_conflicton_on_account() {
    const TEST_NUM_THREADS: usize = 4;
    let mut locks = ThreadAwareLocks::new(TEST_NUM_THREADS);

    let pk1 = Pubkey::new_unique();
    locks.read_account_lock(pk1, 2);
    locks.write_lock_account(pk1, 1);
}

// outstanding write lock must be on same thread
#[test]
#[should_panic(expected="outstanding write lock must be on same thread")]
fn test_read_write_conflicton_on_account_2() {
    const TEST_NUM_THREADS: usize = 4;
    let mut locks = ThreadAwareLocks::new(TEST_NUM_THREADS);

    let pk1 = Pubkey::new_unique();
    locks.write_lock_account(pk1, 1);
    locks.read_account_lock(pk1, 2);
}


// this write lock must be on the same thread
#[test]
#[should_panic(expected="this write lock must be on the same thread")]
fn test_conflict_account_write_lock() {
    const TEST_NUM_THREADS: usize = 4;
    let mut locks = ThreadAwareLocks::new(TEST_NUM_THREADS);

    let pk1 = Pubkey::new_unique();
    locks.write_lock_account(pk1, 1);
    locks.write_lock_account(pk1, 2);
    println!("{:?}",locks);

    assert_eq!(
        locks.locks.get(&pk1).unwrap().write_lock.as_ref().unwrap().lock_count,
        1
    )
}


//// SCHEDULING (THREAD-SET) TEST ////

