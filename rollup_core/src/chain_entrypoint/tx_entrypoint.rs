use std::{collections::HashMap, default};

use solana_sdk::{blake3::Hash, pubkey::Pubkey};
use crate::{line_up_queue::line_up_queue::{AccountInvolvedInTransaction, LineUpQueue}, scheduler::read_write_locks::ThreadAwareLocks};


pub struct  MakeTransaction {
    id : u64,
    tx_type : String,
    accounts : AccountInvolvedInTransaction,
    priority_level : u64
}

pub struct ChainTransaction {
    pub chain_transaction  : HashMap<u64, MakeTransaction>
}


pub struct TransactionsOnThread {
    pub trnasaction_on_thread  : HashMap<u64,usize>
}

impl TransactionsOnThread {
    fn init() -> Self {
        let trnasaction_on_thread_hashmp = HashMap::new();
        TransactionsOnThread {
            trnasaction_on_thread : trnasaction_on_thread_hashmp
        }
    }

    // thread_id -> trnasaction_id
    pub fn get_all_tx_ids_for_thread(&self, thread_id: usize) -> Vec<u64> {
        let mut result = Vec::new();
        for (&id, &tid) in self.trnasaction_on_thread.iter() {
            if tid == thread_id {
                result.push(id);
            }
        }
        result
    }
}

impl Default for ChainTransaction  {
    fn default() -> Self {
        let chain_trnasaction = HashMap::new();
        ChainTransaction {
            chain_transaction : chain_trnasaction
        }
    }
}

impl ChainTransaction {

    pub fn create_new_transaction(&mut self,id:u64,tx_type : String, account : AccountInvolvedInTransaction , priority : u64) -> MakeTransaction  {
        self.chain_transaction.insert(id, MakeTransaction {
            id : id,
            tx_type : tx_type.clone(),
            accounts : AccountInvolvedInTransaction {
                is_writeable_accounts : account.is_writeable_accounts.clone(),
                non_writeable_accounts : account.non_writeable_accounts.clone()
            },
            priority_level : priority
        });   
        MakeTransaction {
            id : id,
            tx_type : tx_type,
            accounts : AccountInvolvedInTransaction {
                is_writeable_accounts : account.is_writeable_accounts,
                non_writeable_accounts : account.non_writeable_accounts
            },
            priority_level : priority 
        }     
    }

    pub fn push_new_transaction_to_the_main_queue(&mut self, lineup_queue : &mut LineUpQueue) {
        //create a new transaction and get everything to put in the add_queue func.
        let new_transaction = self.create_new_transaction(1, "transfer".to_string(), AccountInvolvedInTransaction{
            is_writeable_accounts : vec![],
            non_writeable_accounts : vec![]
        },1);

        lineup_queue.add_to_main_tx_queue( 
            new_transaction.id,
            new_transaction.tx_type,
            new_transaction.accounts,
            new_transaction.priority_level
        );
    }

    pub fn put_all_the_transaction_in_the_lineup_queue(lineup_queue : &mut LineUpQueue) {
        lineup_queue.add_to_line_up();
    }

    pub fn sort_transaction_in_lineup_queue_by_priority(lineup_queue : &mut LineUpQueue) {
        lineup_queue.sort_linup_queue_according_to_priority();
    }

    pub fn clear_lineup_queue(lineup_queue : &mut LineUpQueue) {
        lineup_queue.clear_lineup_queue_for_next_batch();
    }

    //IMP -> all the clone stuff
    // full-up the transaction from lineup_queue and apply RW locks and schedule on threads
    pub fn take_out_individual_transaction_and_apply_RWlocks(lineup_queue : &mut LineUpQueue, thread_aware_locks : &mut ThreadAwareLocks) {
        let transactions: Vec<_> = lineup_queue.lineup_queue.iter().cloned().collect();
        //TODO:
        //do not init trnasaction_on_thread as default, use existing value
        let mut trnasaction_on_thread =  TransactionsOnThread::init();
        for transaction in transactions {

            let is_writeable_accounts_clone = transaction.txs_accounts.is_writeable_accounts.clone();
            let non_writeable_accounts_clone = transaction.txs_accounts.non_writeable_accounts.clone();

            if let Some(scheduled_thread) = thread_aware_locks.try_lock_account(is_writeable_accounts_clone.clone(), non_writeable_accounts_clone.clone()) {
                trnasaction_on_thread.trnasaction_on_thread.insert(transaction.id, scheduled_thread);
            }
            else {
                lineup_queue.add_transaction_to_non_rescheduable_container(transaction.id, transaction.tx_type, transaction.txs_accounts, transaction.priority);
            }
            
        }
    }

    pub fn get_single_transaction_on_a_particular_thread(&self, id : u64) -> &MakeTransaction {
        let transaction = self.chain_transaction.get(&id).unwrap();
        transaction
    }

    pub fn get_all_transaction_on_a_thread(&mut  self,  tsx_on_thread : TransactionsOnThread, thread_id : usize) -> Vec<&MakeTransaction> {
        let ids = TransactionsOnThread::get_all_tx_ids_for_thread(&tsx_on_thread, thread_id);
        let mut all_transaction_on_thread_id = Vec::new();
        for id in ids { 
            let single_txs = self.get_single_transaction_on_a_particular_thread(id);
            all_transaction_on_thread_id.push(single_txs);
        }
        all_transaction_on_thread_id
    }

    pub fn get_all_transaction_from_all_4_thread(&mut self, tsx_on_thread : TransactionsOnThread) {
        let transaction_on_thread_1  = self.get_all_transaction_on_a_thread(tsx_on_thread, 1);
        // let transaction_on_thread_2  = self.get_all_transaction_on_a_thread(tsx_on_thread, 2);
        // let transaction_on_thread_3  = self.get_all_transaction_on_a_thread(tsx_on_thread, 3);
        // let transaction_on_thread_4  = self.get_all_transaction_on_a_thread(tsx_on_thread, 4);
    }



}