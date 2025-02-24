use std::{collections::HashMap, default, hash::DefaultHasher};
use std::hash::{Hash, Hasher};
use solana_sdk::account;
use solana_sdk::instruction::InstructionError;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
use crate::{line_up_queue::line_up_queue::{AccountInvolvedInTransaction, LineUpQueue}, processor::{engine::PayTubeChannel, setup::{system_account, TestValidatorContext}, transaction::{TransactionMetadata, TransactionType}}, scheduler::read_write_locks::{ThreadAwareLocks, ThreadLoadCounter}, users_handler::user_handler::AppUserBase};


// hash done
// eunum done 
// handle duplicate tsx
// handle duplicate accounts
        // Finds the index of each account in the instruction by its pubkey.
        // Then normalizes / unifies the privileges of duplicate accounts.
// fetch stage
// sig verify
// shimmer algo


#[derive(Debug)]
pub struct AccountsMeta {
    pub key : Pubkey,
    pub is_writeable : bool,
    pub is_signer : bool
}

#[derive(Debug)]
pub struct  MakeTransaction {
    pub id : u64,
    pub accounts : AccountInvolvedInTransaction,
    pub priority_level : u64,
    pub transaction_metadata : TransactionMetadata,
    pub from_key : Keypair,
}


#[derive(Debug)]
pub struct ChainTransaction {
    pub chain_transaction  : HashMap<u64, MakeTransaction>
}

#[derive(Clone)]
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

impl AccountsMeta {
    pub fn create_new_meta_with_signer(pubkey : Pubkey,is_writeable : bool) -> Self {
        AccountsMeta {
            key : pubkey,
            is_writeable,
            is_signer : true
        }
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

    pub fn convert_account_meta_to_acc_inv_txs(&mut self,accounts : Vec<AccountsMeta>) -> AccountInvolvedInTransaction {
        let mut writeable_accounts = Vec::new();
        let mut non_writeable_accounts  =  Vec::new();

        for account in accounts {
            if account.is_writeable {
                writeable_accounts.push(account.key);
            } else {
                non_writeable_accounts.push(account.key);
            }
        }

        AccountInvolvedInTransaction {
            is_writeable_accounts : writeable_accounts,
            non_writeable_accounts : non_writeable_accounts
        }
}   

    // pub fn account_previlage_normalization_and_previlage_checker(&mut self,accounts : Vec<AccountsMeta>) {
    //     let mut duplicated_instruction_account : Vec<InstructionAccount> = Vec::new();
    //     let mut duplicate_indicies : Vec<usize> = Vec::new();
        
    //     for (instruction_account_account_index , account_meta) in accounts.iter().enumerate() {
    //         let index_in_transaction = instruction_account_account_index;

    //         if let Some(duplicate_index) = duplicated_instruction_account
    //         .iter()
    //         .position(|instruction_account| {
    //             instruction_account.index_in_transaction == index_in_transaction
    //         }) {
    //             duplicate_indicies.push(duplicate_index);
    //             let instruction_account = duplicated_instruction_account
    //             .get_mut(duplicate_index)
    //             .ok_or(InstructionError::NotEnoughAccountKeys)?;
    //         instruction_account.is_signer |= account_meta.is_signer;
    //         instruction_account.is_writeable |= account_meta.is_writeable;
    //         } else {
    //             let index_in_caller = 0;
    //             duplicate_indicies.push(duplicated_instruction_account.len());
    //             duplicated_instruction_account.push(InstructionAccount {
    //                 index_in_transaction : index_in_transaction,
    //                 index_in_caller,
    //                 index_in_callee : instruction_account_account_index,
    //                 is_signer : account_meta.is_signer,
    //                 is_writeable : account_meta.is_writeable
    //             });
    //         }

    //     }
    // }



    pub fn create_new_transaction(&mut self,id:u64, accounts : AccountInvolvedInTransaction , priority : u64 , transaction_metadata : TransactionMetadata , user: &mut AppUserBase , program_id : Pubkey , user_name : String) -> MakeTransaction  {
        let from_key = user.get_keypair_from_user_name(program_id, user_name);
        println!("{:?}",from_key.pubkey());
        self.chain_transaction.insert(id, MakeTransaction {
            id : id,
            accounts : accounts.clone(),
            priority_level : priority,
            transaction_metadata : transaction_metadata.clone(),
            from_key : from_key.insecure_clone(),

        });   
        MakeTransaction {
            id : id,
            accounts : accounts,
            priority_level : priority,
            transaction_metadata,
            from_key: from_key.insecure_clone(),
        }     
    }

    pub fn push_new_transaction_to_the_main_queue(&mut self, lineup_queue : &mut LineUpQueue, account : Vec<AccountsMeta> , transaction_metadata : TransactionMetadata, app_user_base : &mut AppUserBase , program_id : Pubkey , user_name : String) {
        //create a new transaction and get everything to put in the add_queue func.
        let hashed_id = self.create_hash(transaction_metadata.clone());
        let account_involved_in_transaction = self.convert_account_meta_to_acc_inv_txs(account);
        let new_transaction = self.create_new_transaction(hashed_id, account_involved_in_transaction ,1 , 
        match transaction_metadata.txs_type {
            TransactionType::Transfer => {
                TransactionMetadata {
                    txs_type : TransactionType::Transfer,
                    keys : vec![
                    transaction_metadata.keys[0],
                    transaction_metadata.keys[1],
                    transaction_metadata.keys[2],
                    transaction_metadata.keys[3]
                    ],
                    args : vec![
                        transaction_metadata.args[0]
                    ]
                }
            }
        },
        app_user_base,
        program_id,
        user_name
    );

        lineup_queue.add_to_main_tx_queue( 
            new_transaction.id,
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

    pub fn create_hash(&mut self,tx_metadata : TransactionMetadata) -> u64 {
        let mut hasher = DefaultHasher::new();
        tx_metadata.hash(&mut hasher);
        hasher.finish()
    }

    pub fn clear_lineup_queue(&mut self,lineup_queue : &mut LineUpQueue) {
        lineup_queue.clear_lineup_queue_for_next_batch();
    }

    //Optimiization while scheduling
    // pub fn normalize_accounts_previalges(&mut self , write_prevelaged_accounts : Vec<Pubkey> , mut read_prevelaged_account : Vec<Pubkey>)  -> Vec<Pubkey> {
    //     for account in read_prevelaged_account.clone() {
    //         if write_prevelaged_accounts.contains(&account) {
    //             let index = read_prevelaged_account.iter().position(|&key| &key == &account).unwrap();
    //             read_prevelaged_account.remove(index);
    //         }
    //     }
    //     read_prevelaged_account
    // }

    
    pub fn take_out_individual_transaction_and_apply_RWlocks(&mut self,lineup_queue : &mut LineUpQueue, thread_aware_locks : &mut ThreadAwareLocks , transaction_on_thread : &mut TransactionsOnThread , thread_load_counter : &mut ThreadLoadCounter) {
        let transactions: Vec<_> = lineup_queue.lineup_queue.iter().cloned().collect();

        for transaction in transactions {

            let is_writeable_accounts_clone = transaction.txs_accounts.is_writeable_accounts.clone();
            let non_writeable_accounts_clone = transaction.txs_accounts.non_writeable_accounts.clone();
            //let normalized_non_writeable_accounts = self.normalize_accounts_previalges(is_writeable_accounts_clone.clone(), non_writeable_accounts_clone);

            if let Some(scheduled_thread) = thread_aware_locks.try_lock_account(is_writeable_accounts_clone.clone(), non_writeable_accounts_clone.clone(),thread_load_counter) {
                transaction_on_thread.trnasaction_on_thread.insert(transaction.id, scheduled_thread);
            }
            else {
                lineup_queue.add_transaction_to_non_rescheduable_container(transaction.id, transaction.txs_accounts, transaction.priority);
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

    
    pub fn process_all_transaction_from_threads(&mut self, tsx_on_thread : TransactionsOnThread , thread_id : usize) {
        let transaction_on_thread_1 = self.get_all_transaction_on_a_thread(tsx_on_thread, thread_id);
        println!("side_tx_res{:?}",transaction_on_thread_1);
        let transaction_metadata = get_all_transaction_metadata_from_transaction(transaction_on_thread_1.clone());
        let final_transaction_metadata  = transaction_metadata.as_slice();


        let accounts  = prepare_account_for_the_transaction(transaction_on_thread_1.clone());
    
        let context = TestValidatorContext::start_with_accounts(accounts);
        let test_validator = &context.test_validator;
        let payer = context.payer.insecure_clone();
    
        let rpc_client = test_validator.get_rpc_client();
        let transaction_keys = prepare_key_for_trnasactions(transaction_on_thread_1, payer);
        let paytube_channel = PayTubeChannel::new(transaction_keys, rpc_client);

        println!("metadata {:?}", final_transaction_metadata);
        paytube_channel.process_paytube_transfers(final_transaction_metadata);
        
    }

    pub fn process_all_transactions(&mut self,tsx_on_thread : TransactionsOnThread ) {
        self.process_all_transaction_from_threads(tsx_on_thread.clone(), 1);
        self.process_all_transaction_from_threads(tsx_on_thread.clone(), 2);
    }

    
}
pub fn get_all_transaction_metadata_from_transaction(transaction : Vec<&MakeTransaction>) -> Vec<TransactionMetadata> {
    let mut metadata_vec : Vec<TransactionMetadata> = Vec::new();
    let mut transaction_metadata; 
    for transaction in transaction {
        match transaction.transaction_metadata.txs_type { 
            TransactionType::Transfer => {
                transaction_metadata = TransactionMetadata {
                 txs_type : TransactionType::Transfer,   
                 keys : vec![
                    transaction.transaction_metadata.keys[0],
                    transaction.transaction_metadata.keys[1],
                    transaction.transaction_metadata.keys[2],
                    transaction.transaction_metadata.keys[3]
                 ],
                 args : vec![
                    transaction.transaction_metadata.args[0]
                 ]
                };
            }
        }
        metadata_vec.push(transaction_metadata);
    }
    metadata_vec
}

pub fn prepare_key_for_trnasactions(transaction : Vec<&MakeTransaction> , payer : Keypair) -> Vec<Keypair>{
       
    let mut key_vec : Vec<Keypair> = vec![];
    key_vec.push(payer);
    for transaction in transaction {
        key_vec.push(transaction.from_key.insecure_clone());
        
    }
    key_vec
}

pub fn prepare_account_for_the_transaction(transaction : Vec<&MakeTransaction>) -> Vec<(Pubkey,solana_sdk::account::AccountSharedData)> {
    let mut account : Vec<(Pubkey, solana_sdk::account::AccountSharedData)> =  vec![];

    for transaction in transaction {
        account.push((transaction.from_key.pubkey() , system_account(50_000_000)));
    }

    account
}