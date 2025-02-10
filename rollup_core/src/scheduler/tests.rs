use solana_sdk::pubkey::Pubkey;

use crate::scheduler::read_write_locks::ThreadLoadCounter;

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
// multi-case

#[test]
fn test_schedule_on_thread_with_only_write() {
    const TEST_NUM_THREADS: usize = 4;
    let mut locks = ThreadAwareLocks::new(TEST_NUM_THREADS);
    let mut thread_load_counter = ThreadLoadCounter::default();
    let pk1 = Pubkey::new_unique();
    locks.write_lock_account(pk1, 1);

    let schedulable_thread = locks.schedule_on_threads(pk1,true,&mut thread_load_counter);
    println!("{:?}",schedulable_thread)
}

#[test]
fn test_schedule_on_thread_with_read_and_write() {
    const TEST_NUM_THREADS: usize = 4;
    let mut locks = ThreadAwareLocks::new(TEST_NUM_THREADS);
    let mut thread_load_counter = ThreadLoadCounter::default();
    let pk1 = Pubkey::new_unique();
    locks.write_lock_account(pk1, 1);
    locks.read_account_lock(pk1, 1);
    let schedulable_thread = locks.schedule_on_threads(pk1,true,&mut thread_load_counter);
    assert_eq!(
        schedulable_thread,
        Some(1)
    );
    println!("{:?}",schedulable_thread)
}

//test_accounts_schedulable_threads_outstanding_read_only

#[test]
fn test_accounts_schedulable_threads_1() {
    let pk1 = Pubkey::new_unique();
    let pk2 = Pubkey::new_unique(); 

    const TEST_NUM_THREADS: usize = 4;
    let mut locks = ThreadAwareLocks::new(TEST_NUM_THREADS);
    let mut thread_load_counter = ThreadLoadCounter::default();
    locks.read_account_lock(pk1, 2);
    let scheduable_thread_for_new_tsx = locks.accounts_schedulable_threads(vec![pk1,pk2] , vec![],&mut thread_load_counter);

    assert_eq!(
        locks.simplefy_threads(scheduable_thread_for_new_tsx.clone(),&mut thread_load_counter).len(),
        1
    );
    assert_eq!(
        locks.simplefy_threads(scheduable_thread_for_new_tsx.clone(),&mut thread_load_counter)[0],
        2
    );

    println!("{:?}",locks.simplefy_threads(scheduable_thread_for_new_tsx,&mut thread_load_counter))

}

#[test]
fn test_accounts_schedulable_threads_2() {
    let pk1 = Pubkey::new_unique();
    let pk2 = Pubkey::new_unique(); 
    let ANY_THREAD : usize = 1;
    const TEST_NUM_THREADS: usize = 4;
    let mut locks = ThreadAwareLocks::new(TEST_NUM_THREADS);
    let mut thread_load_counter = ThreadLoadCounter::default();
    locks.read_account_lock(pk1, 2);
    let scheduable_thread_for_new_tsx = locks.accounts_schedulable_threads(vec![] , vec![pk1,pk2],&mut thread_load_counter);

    assert_eq!(
        locks.simplefy_threads(scheduable_thread_for_new_tsx.clone(),&mut thread_load_counter).len(),
        1
    );
    assert_eq!(
        locks.simplefy_threads(scheduable_thread_for_new_tsx.clone(),&mut thread_load_counter)[0],
        ANY_THREAD
    );

    println!("{:?}",locks.simplefy_threads(scheduable_thread_for_new_tsx,&mut thread_load_counter))

}

#[test]
#[should_panic(expected="Cannot schedule because of multi-threading conflict")]
fn test_accounts_schedulable_threads_3() {
    let pk1 = Pubkey::new_unique();
    let pk2 = Pubkey::new_unique(); 
    const TEST_NUM_THREADS: usize = 4;
    let mut locks = ThreadAwareLocks::new(TEST_NUM_THREADS);
    let mut thread_load_counter = ThreadLoadCounter::default();
    locks.read_account_lock(pk1, 1);
    locks.read_account_lock(pk1, 2);
    let _scheduable_thread_for_new_tsx = locks.accounts_schedulable_threads(vec![pk1,pk2] , vec![],&mut thread_load_counter);
}

#[test]
#[should_panic(expected="Cannot schedule because of multi-threading conflict")]
fn test_try_lock_account_with_conflict() {
    let pk1 = Pubkey::new_unique();
    let pk2 = Pubkey::new_unique(); 
    const TEST_NUM_THREADS: usize = 4;
    let mut locks = ThreadAwareLocks::new(TEST_NUM_THREADS);
    let mut thread_load_counter = ThreadLoadCounter::default();
    locks.read_account_lock(pk1, 2);
    locks.read_account_lock(pk1, 3);

    locks.try_lock_account(vec![pk1], vec![pk2],&mut thread_load_counter);
} 

#[test]
fn test_try_lock_account_with_no_conflict() {
    let pk1 = Pubkey::new_unique();
    let pk2 = Pubkey::new_unique(); 
    const TEST_NUM_THREADS: usize = 4;
    let mut locks = ThreadAwareLocks::new(TEST_NUM_THREADS);
    let mut thread_load_counter = ThreadLoadCounter::default();
    locks.write_lock_account(pk1, 2);

    assert_eq!(
        locks.try_lock_account(vec![pk1], vec![pk2],&mut thread_load_counter),
        Some(2)
    )

} 

#[test]
fn test_randomness() {
    let pk1 = Pubkey::new_unique();
    let pk2 = Pubkey::new_unique(); 
    let pk3 = Pubkey::new_unique(); 
    //let pk4 = Pubkey::new_unique();
    const TEST_NUM_THREADS: usize = 4;
    let mut locks = ThreadAwareLocks::new(TEST_NUM_THREADS);
    let mut thread_load_counter = ThreadLoadCounter::default();
    let tx1 = locks.try_lock_account(vec![], vec![pk1],&mut thread_load_counter);
    let tx2 = locks.try_lock_account(vec![], vec![pk1],&mut thread_load_counter);
    let tx3 = locks.try_lock_account(vec![], vec![pk2], &mut thread_load_counter);
    let tx4 = locks.try_lock_account(vec![pk3], vec![], &mut thread_load_counter);
    let tx5 = locks.try_lock_account(vec![pk3], vec![], &mut thread_load_counter);

    //let tx6 = locks.try_lock_account(vec![pk4], vec![], &mut thread_load_counter);
    //let tx_err = locks.try_lock_account(vec![pk1], vec![], &mut thread_load_counter);

    println!("{:?}",tx1);
    println!("{:?}",tx2);
    println!("{:?}",tx3);
    println!("{:?}",tx4);
    println!("{:?}",tx5);
    //println!("{:?}",tx6);

    println!("{:?}",thread_load_counter.load_counter)
}

#[test]
fn test_load_increaser() {
    let mut thread_load_counter = ThreadLoadCounter::default();
    
    thread_load_counter.increase_load_count(1);
    thread_load_counter.increase_load_count(2);
    thread_load_counter.increase_load_count(3);

    println!("{:?}",thread_load_counter)
}