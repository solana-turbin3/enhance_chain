use std::collections::HashMap;

use solana_sdk::pubkey::Pubkey;

/// Identifier for a thread
pub type ThreadId = usize;

type LockCount = u32;

pub struct AccountWriteLocks {
    thread_id : ThreadId,
    lock_count : LockCount
}

pub struct AccountReadLocks {
    thread_id : ThreadId,
    lock_count : LockCount
}

pub struct AccountLocks {
    pub write_lock : Option<AccountWriteLocks>,
    pub read_lock : Option<AccountReadLocks>
}

pub struct ThreadAwareLocks {
    number_of_thread : usize,
    locks : HashMap<Pubkey,AccountLocks>
}

