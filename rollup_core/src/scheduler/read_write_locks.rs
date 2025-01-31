use std::collections::HashMap;
use solana_sdk::pubkey::Pubkey;

/// Identifier for a thread
const MAX_THREAD:usize = 4;
pub type ThreadId = usize;
type LockCount = u32;

pub struct AccountWriteLocks {
    thread_id : ThreadId,
    lock_count : LockCount
}

pub struct AccountReadLocks {
    thread_set : [bool;MAX_THREAD],
    lock_count : [LockCount;MAX_THREAD]
}

pub struct AccountLocks {
    pub write_lock : Option<AccountWriteLocks>,
    pub read_lock : Option<AccountReadLocks>
}

pub struct ThreadAwareLocks {
    number_of_thread : usize,
    locks : HashMap<Pubkey,AccountLocks>
}

impl ThreadAwareLocks {

    pub fn init(number_of_thread:usize) -> Self {
        Self {
            number_of_thread,
            locks : HashMap::new()
        }
    }
    pub fn write_lock_account(&mut self, account:Pubkey, thread_id : ThreadId) {
    
    // or_insert*
     let entry = self.locks.entry(account).or_insert(AccountLocks {
         write_lock: None,
         read_lock: None,
     });
     let AccountLocks {
        write_lock,
        read_lock
     } = entry;

     // if onw thread is writing on an account then the 
     // other thread shouldnt be reading from it, it should be on the same thread
     if let Some(read_lock) = read_lock {
        let mut thread_set  = [false,false,false,false];
        thread_set[thread_id] = true;
        assert_eq!(
            read_lock.thread_set,
            thread_set,
            "outstanding read lock must be on same thread"
        )
     }

     if let Some(write_lock) = write_lock {

        assert_eq!(
            write_lock.thread_id,
            thread_id,
            "this write lock must be on the same thread"
        );

        write_lock.lock_count +=1;

     } else {
        *write_lock = Some(AccountWriteLocks {
            thread_id,
            lock_count : 1
        })
     }
    }
    pub fn read_account_lock(&mut self, account: Pubkey, thread_id: ThreadId) {
       let entry = self.locks.entry(account).or_insert(
        AccountLocks {
            read_lock : None,
            write_lock : None
        }
    );
        let AccountLocks {
            write_lock,
            read_lock
        } = entry;

        // make sense because what if other thread write on it 
        // and one thread is reading from it
        // thats why outstanding write_lock should be on the same thread
        if let Some(write_lock) = write_lock {
            assert_eq!(
                write_lock.thread_id,
                thread_id,
                "outstanding write lock must be on same thread"
            )
        }

       if let Some(read_lock) = read_lock {
            read_lock.thread_set[thread_id] = true;
            read_lock.lock_count[thread_id] +=1
       } else {
        let mut initial_thread_set = [false,false,false,false];
        let mut  lock_count = [0;MAX_THREAD];
        lock_count[thread_id] +=1;
        initial_thread_set[thread_id] = true;
        *read_lock = Some(AccountReadLocks {
            thread_set : initial_thread_set,
            lock_count 
        })
       }
    }
}


