use std::{collections::HashMap, default};

use solana_sdk::{blake3::Hash, pubkey::Pubkey, signature::Keypair, signer::Signer};
use crate::{line_up_queue::line_up_queue::{AccountInvolvedInTransaction, LineUpQueue}, processor::{engine::PayTubeChannel, setup::{system_account, TestValidatorContext}, transaction::ForTransferTransaction}, scheduler::read_write_locks::ThreadAwareLocks, users_handler::user_handler::AppUserBase};

// #[derive(Clone)]
// pub struct TransferTransactionMetadata {
//     pub mint: Option<Pubkey>,
//     pub from: Pubkey,
//     pub to: Pubkey,
//     pub amount: u64,
// }

#[derive(Debug)]
pub struct  MakeTransaction {
    pub id : u64,
    pub tx_type : String,
    pub accounts : AccountInvolvedInTransaction,
    pub priority_level : u64,
    pub transaction_metadata : ForTransferTransaction,
    pub from_key : Keypair,
}

#[derive(Debug)]
pub struct ChainTransaction {
    pub chain_transaction  : HashMap<u64, MakeTransaction>
}


pub struct TransactionsOnThread {
    pub trnasaction_on_thread  : HashMap<u64,usize>
}

impl Default for TransactionsOnThread {
    fn default() -> Self {
        let trnasaction_on_thread_hashmp = HashMap::new();
        TransactionsOnThread {
            trnasaction_on_thread : trnasaction_on_thread_hashmp
        }
    }
}

impl TransactionsOnThread {

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

    pub fn create_new_transaction(&mut self,id:u64,tx_type : String, account : AccountInvolvedInTransaction , priority : u64 , transaction_metadata : ForTransferTransaction , user: &mut AppUserBase , program_id : Pubkey , user_name : String) -> MakeTransaction  {
        let from_key = user.get_keypair_from_user_name(program_id, user_name);
        println!("{:?}",from_key.pubkey());
        self.chain_transaction.insert(id, MakeTransaction {
            id : id,
            tx_type : tx_type.clone(),
            accounts : AccountInvolvedInTransaction {
                is_writeable_accounts : account.is_writeable_accounts.clone(),
                non_writeable_accounts : account.non_writeable_accounts.clone()
            },
            priority_level : priority,
            transaction_metadata : transaction_metadata.clone(),
            from_key : from_key.insecure_clone(),

        });   
        MakeTransaction {
            id : id,
            tx_type : tx_type,
            accounts : AccountInvolvedInTransaction {
                is_writeable_accounts : account.is_writeable_accounts,
                non_writeable_accounts : account.non_writeable_accounts
            },
            priority_level : priority,
            transaction_metadata,
            from_key: from_key.insecure_clone(),
        }     
    }

    pub fn push_new_transaction_to_the_main_queue(&mut self, lineup_queue : &mut LineUpQueue, account : AccountInvolvedInTransaction , transaction_metadata : ForTransferTransaction, app_user_base : &mut AppUserBase , program_id : Pubkey , user_name : String) {
        //create a new transaction and get everything to put in the add_queue func.
        let new_transaction = self.create_new_transaction(1, "transfer".to_string(), AccountInvolvedInTransaction{
            is_writeable_accounts : account.is_writeable_accounts,
            non_writeable_accounts : account.non_writeable_accounts
        },1 , ForTransferTransaction {
            mint : Some(transaction_metadata.mint).unwrap(),
            from : transaction_metadata.from,
            to : transaction_metadata.to,
            amount : transaction_metadata.amount
        },
        app_user_base,
        program_id,
        user_name
    );

        lineup_queue.add_to_main_tx_queue( 
            new_transaction.id,
            new_transaction.tx_type,
            new_transaction.accounts,
            new_transaction.priority_level
        );
    }

    pub fn put_all_the_transaction_in_the_lineup_queue(&mut self,lineup_queue : &mut LineUpQueue) {
        lineup_queue.add_to_line_up();
    }

    pub fn sort_transaction_in_lineup_queue_by_priority(&mut self ,lineup_queue : &mut LineUpQueue) {
        lineup_queue.sort_linup_queue_according_to_priority(true);
    }

    pub fn clear_lineup_queue(&mut self,lineup_queue : &mut LineUpQueue) {
        lineup_queue.clear_lineup_queue_for_next_batch();
    }

    // full-up the transaction from lineup_queue and apply RW locks and schedule on threads
    //IMP -> all the clone stuff
    pub fn take_out_individual_transaction_and_apply_RWlocks(&mut self,lineup_queue : &mut LineUpQueue, thread_aware_locks : &mut ThreadAwareLocks , transaction_on_thread : &mut TransactionsOnThread) {
        let transactions: Vec<_> = lineup_queue.lineup_queue.iter().cloned().collect();
        //TODO:
        //do not init trnasaction_on_thread as default, use existing value
        // let mut trnasaction_on_thread =  TransactionsOnThread::default();
        for transaction in transactions {

            let is_writeable_accounts_clone = transaction.txs_accounts.is_writeable_accounts.clone();
            let non_writeable_accounts_clone = transaction.txs_accounts.non_writeable_accounts.clone();

            if let Some(scheduled_thread) = thread_aware_locks.try_lock_account(is_writeable_accounts_clone.clone(), non_writeable_accounts_clone.clone()) {
                transaction_on_thread.trnasaction_on_thread.insert(transaction.id, scheduled_thread);
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
        let ids: Vec<u64> = TransactionsOnThread::get_all_tx_ids_for_thread(&tsx_on_thread, thread_id);
        let mut all_transaction_on_thread_id = Vec::new();
        for id in ids { 
            let single_txs = self.get_single_transaction_on_a_particular_thread(id);
            all_transaction_on_thread_id.push(single_txs);
        }
        all_transaction_on_thread_id
    }

    
    pub fn process_all_transaction_from_thread_1(&mut self, tsx_on_thread : TransactionsOnThread) {
        let transaction_on_thread_1 = self.get_all_transaction_on_a_thread(tsx_on_thread, 1);
        println!("side_tx_res{:?}",transaction_on_thread_1);
        let transaction_metadata = get_all_transaction_metadata_from_transaction(transaction_on_thread_1.clone());
        let final_transaction_metadata  = transaction_metadata.as_slice();

        let from_key = transaction_on_thread_1[0].from_key.insecure_clone();
        println!("fromkey{:?}",from_key.pubkey());
        // let to_key = transaction_on_thread_1[0].to_key.insecure_clone();
        // let alice = Keypair::new();
        // let bob = Keypair::new();

        // let alice_pubkey = alice.pubkey();
        // let bob_pubkey = bob.pubkey();
    
        let accounts: Vec<(Pubkey, solana_sdk::account::AccountSharedData)> = vec![
            (from_key.pubkey(), system_account(10_000_000)),
            // (to_key.pubkey(), system_account(10_000_000)),
        ];
    
        let context = TestValidatorContext::start_with_accounts(accounts);
        let test_validator = &context.test_validator;
        let payer = context.payer.insecure_clone();
    
        let rpc_client = test_validator.get_rpc_client();
        
        let paytube_channel = PayTubeChannel::new(vec![payer , from_key.insecure_clone()], rpc_client);

        // println!("metadata {:?}", final_transaction_metadata);

        paytube_channel.process_paytube_transfers(&[
            ForTransferTransaction {
                from : transaction_metadata[0].from,
                to : transaction_metadata[0].to,
                amount : 2_000_000,
                mint : None
            }
        ]);
        // paytube_channel.process_paytube_transfers(final_transaction_metadata);

        // paytube_channel.process_paytube_transfers(&[
        //     // Alice -> Bob 2_000_000
        //     ForTransferTransaction {
        //         from: alice_pubkey,
        //         to: bob_pubkey,
        //         amount: 2_000_000,
        //         mint: None,
        //     },
        //     // Bob -> Will 5_000_000
        //     ForTransferTransaction {
        //         from: bob_pubkey,
        //         to: will_pubkey,
        //         amount: 5_000_000,
        //         mint: None,
        //     },
        //     // Alice -> Bob 2_000_000
        //     ForTransferTransaction {
        //         from: alice_pubkey,
        //         to: bob_pubkey,
        //         amount: 2_000_000,
        //         mint: None,
        //     },
        //     // Will -> Alice 1_000_000
        //     ForTransferTransaction {
        //         from: will_pubkey,
        //         to: alice_pubkey,
        //         amount: 1_000_000,
        //         mint: None,
        //     },
        // ]);
        
    }
    
    
    
}
pub fn get_all_transaction_metadata_from_transaction(transaction : Vec<&MakeTransaction>) -> Vec<ForTransferTransaction> {
    let mut metadata_vec : Vec<ForTransferTransaction> = Vec::new();
    for transaction in transaction {
        let transaction_metadata = ForTransferTransaction {
           mint : transaction.transaction_metadata.mint,
          to : transaction.transaction_metadata.to,
          from : transaction.transaction_metadata.from,
          amount : transaction.transaction_metadata.amount 
        };
        metadata_vec.push(transaction_metadata);
    }
    metadata_vec
}