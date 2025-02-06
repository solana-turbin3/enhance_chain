use solana_sdk::pubkey::Pubkey;
use crate::{line_up_queue::line_up_queue::{AccountInvolvedInTransaction, LineUpQueue}, scheduler::read_write_locks::ThreadAwareLocks};

pub struct  MakeTransaction {
    id : u64,
    tx_type : String,
    accounts : AccountInvolvedInTransaction,
    priority_level : u64
}

impl MakeTransaction {
    pub fn create_new_transaction(&mut self,id:u64,tx_type : String, account : AccountInvolvedInTransaction , priority : u64) -> Self {
        Self {
            id : id,
            tx_type : tx_type,
            accounts : AccountInvolvedInTransaction {
                is_writeable_accounts : vec![],
                non_writeable_accounts : vec![]
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
        for transaction in transactions {

            let is_writeable_accounts_clone = transaction.txs_accounts.is_writeable_accounts.clone();
            let non_writeable_accounts_clone = transaction.txs_accounts.non_writeable_accounts.clone();

            if let Some(scheduled_thread) = thread_aware_locks.try_lock_account(is_writeable_accounts_clone.clone(), non_writeable_accounts_clone.clone()) {
                //TODO:
                //do something with scheduled_thread
            }
            else {
                lineup_queue.add_transaction_to_non_rescheduable_container(transaction.id, transaction.tx_type, transaction.txs_accounts, transaction.priority);
            }
            
        }
    }


}