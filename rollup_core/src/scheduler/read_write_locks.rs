use std::{collections::HashMap, ptr::NonNull};
use solana_sdk::pubkey::Pubkey;

/// Identifier for a thread
const MAX_THREAD:usize = 4;
pub type ThreadId = usize;
type LockCount = u32;

#[derive(Debug)]
pub struct AccountWriteLocks {
    pub thread_id : ThreadId,
    pub lock_count : LockCount
}

#[derive(Debug)]
pub struct AccountReadLocks {
    pub thread_set : [bool;MAX_THREAD],
    pub lock_count : [LockCount;MAX_THREAD]
}

#[derive(Debug)]
pub struct AccountLocks {
    pub write_lock : Option<AccountWriteLocks>,
    pub read_lock : Option<AccountReadLocks>
}

#[derive(Debug)]
pub struct ThreadAwareLocks {
    pub number_of_thread : usize,
    pub locks : HashMap<Pubkey,AccountLocks>
}

impl ThreadAwareLocks {

    pub fn new(number_of_thread:usize) -> Self {
        Self {
            number_of_thread,
            locks : HashMap::new()
        }
    }

    pub fn try_lock_account(
        &mut self,
        write_account : Pubkey,
        read_account : Pubkey,
    ) {
        // let scheduable_threads = self.accounts_schedulable_threads(write_account, read_account);
        // self.lock_account(write_account, read_account, scheduable_threads);
    }

    pub fn accounts_schedulable_threads(
        &mut self,
        write_account : Vec<Pubkey>,
        read_account : Vec<Pubkey>,
    ) -> Vec<usize> {
        let mut accounts_schedulable_threads: Vec<usize> = Vec::new();

        for account in write_account {
            let schedulable_threads = self.schedule_on_threads(account,true);
            accounts_schedulable_threads.push(schedulable_threads.unwrap());
        }
        for account in read_account {
            let schedulable_threads = self.schedule_on_threads(account,false);
            accounts_schedulable_threads.push(schedulable_threads.unwrap());
        }
        accounts_schedulable_threads
    }

    // all the account that doesnt care where the txs goes
    // or account the doesnt have any read or write lock mean can go to any thread
    // for those account im putting thread_id -> 100 (just as a identifier)
    // and here im removing those account thread_id because
    // these account doesnt impact where the parent transaction
    // will be scheduled and to simpleify stuff 
    pub fn simplefy_threads(&mut self,mut account_schedable_thread: Vec<usize>) -> 
    Vec<usize> {
        let any_thread : usize = 1;
        account_schedable_thread.retain(|&thread_id| thread_id != 100);
        // if account_schedable_thread.len() is 0, then the tsx can be scheduled in any of the thread
        if account_schedable_thread.len()==0 {
            return vec![any_thread];
        }
        account_schedable_thread.clone()
    }

    pub fn lock_account(
        &mut self,
        write_account : Vec<Pubkey>,
        read_account : Vec<Pubkey>,
        thread_id : ThreadId
    ) {
        assert!(
            thread_id < self.number_of_thread,
            "thread_id must be < num_threads"
        );

        for account in write_account {
            self.write_lock_account(account, thread_id);
        }


        for account in read_account {
            self.read_account_lock(account, thread_id);
        }
    }

    //1. only read lock applied on the account
    //2. only write lock applied on the account -> write_lock.thread_id
    //3. read and write both locks are applied -> write_lock.thread_id
    //4. none -> none
    pub fn schedule_on_threads(&mut self, account: Pubkey, for_write:bool) -> Option<usize> {
        match self.locks.get(&account) {
            //None -> any/doesnt-care
            None => Some(100),
            Some(AccountLocks {
                write_lock : Some(write_lock),
                read_lock : None
            }) => Some(write_lock.thread_id),
            // for a account, if both read and write
            // locks are there then schedulabe thread should only be one
            // can be related to other errors as well in read&write lock fun as well
            //below
            Some(AccountLocks{
                write_lock : Some(write_lock),
                read_lock : Some(read_lock)
            }) => {
                assert_eq!(
                    write_lock.thread_id,
                    self.convert_thread_set_into_single_thread_id(read_lock)
                );
                Some(write_lock.thread_id)
            },
            Some(AccountLocks{
                write_lock : None,
                read_lock : Some(read_lock)
            }) => {
                let schedulable_thread = self.handle_only_read_condition(read_lock);
                let any_thread = 1;
                if !for_write {
                return Some(any_thread);
                } else {
                    schedulable_thread
                }
            }
            Some(AccountLocks{
                write_lock : None,
                read_lock : None
            }) => unreachable!()
        }
    }

    pub fn convert_thread_set_into_single_thread_id(&self, read_lock: &AccountReadLocks) -> usize {
        read_lock.thread_set.iter().filter(|&&status| status).count()
    }
    

     pub fn handle_only_read_condition(&self, read:&AccountReadLocks) -> Option<usize>{
        // one condition is left that is when write could also happen
        // then if read happening on only thread then its fine
        // but if happening on differnet threads the return None
       let true_indicies : Vec<usize> = read.thread_set.iter()
       .enumerate()
       .filter_map(|(i, &bool)| if bool {Some(i)} else {None})
       .collect();
    let count = true_indicies.len();
       if count == 1 {
       Some(true_indicies[0])
    } else {
        // TODO: handle re-schedule condition
        //None
        panic!("Cannot schedule because of multi-threading conflict")
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

     // if one thread is writing on an account then the 
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


