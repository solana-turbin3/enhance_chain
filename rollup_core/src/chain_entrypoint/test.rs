use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};

use crate::{chain_entrypoint::tx_entrypoint::TransactionsOnThread, line_up_queue::line_up_queue::{AccountInvolvedInTransaction, LineUpQueue}, processor::transaction::TransactionMetadata, scheduler::read_write_locks::{ThreadAwareLocks, ThreadLoadCounter}, users_handler::user_handler::AppUserBase};

use super::tx_entrypoint::ChainTransaction;

#[test]

fn test_full_flow() {

    let mut chain_trnasaction = ChainTransaction::default();
    let mut app_user_base = AppUserBase::default();
    let mut thread_load_counter = ThreadLoadCounter::default();

    let program_id = Keypair::new().pubkey();
    app_user_base.register_app(program_id);
    app_user_base.add_new_user_to_app(program_id);

    assert_eq!(
        app_user_base.app_user_base.get(&program_id).unwrap().users.len(),
        1
    );

    let user_key = app_user_base.get_keypair_from_user_name(program_id, "user1".to_string());

    let w_account = Keypair::new().pubkey();  
    let r_account = Keypair::new().pubkey();
    let to = Keypair::new();

    let transaction_metadata = TransactionMetadata {
        amount : 10_000_000,
        mint : None,
        from : user_key.pubkey(),
        to : to.pubkey()
    };

    let transaction_metadata_2 = TransactionMetadata {
        amount : 10_000_000,
        mint : None,
        from : user_key.pubkey(),
        to : to.pubkey()
    };

     let transaction_metadata_3 = TransactionMetadata {
        amount : 10_000_000,
        mint : None,
        from : user_key.pubkey(),
        to : to.pubkey()
    };

    let transaction_accounts = AccountInvolvedInTransaction {
        is_writeable_accounts : vec![],
        non_writeable_accounts : vec![r_account]
    };


    let transaction_accounts_2 = AccountInvolvedInTransaction {
        is_writeable_accounts : vec![],
        non_writeable_accounts : vec![r_account]
    };

    let transaction_accounts_3 = AccountInvolvedInTransaction {
        is_writeable_accounts : vec![r_account],
        non_writeable_accounts : vec![]
    };

    let mut lineup_queue = LineUpQueue::default();
    let mut thread_aware_locks = ThreadAwareLocks::new(4);
    let mut transaction_on_thread = TransactionsOnThread::default();
    
    chain_trnasaction.push_new_transaction_to_the_main_queue(&mut lineup_queue, transaction_accounts, transaction_metadata , &mut app_user_base,program_id , "user1".to_string(),1);
    chain_trnasaction.push_new_transaction_to_the_main_queue(&mut lineup_queue, transaction_accounts_2, transaction_metadata_2 , &mut app_user_base,program_id , "user1".to_string(),2);
    chain_trnasaction.push_new_transaction_to_the_main_queue(&mut lineup_queue, transaction_accounts_3, transaction_metadata_3 , &mut app_user_base,program_id , "user1".to_string(),3);

   chain_trnasaction.put_all_the_transaction_in_the_lineup_queue(&mut lineup_queue);

   chain_trnasaction.sort_transaction_in_lineup_queue_by_priority(&mut lineup_queue);

   assert_eq!(
    lineup_queue.lineup_queue.len(),
    3
   );

   assert_eq!(
    chain_trnasaction.chain_transaction.len(),
    3
   );

   chain_trnasaction.take_out_individual_transaction_and_apply_RWlocks(&mut lineup_queue, &mut thread_aware_locks,&mut transaction_on_thread,&mut thread_load_counter);
   
   assert_eq!(
    transaction_on_thread.trnasaction_on_thread.len(),
    2
   );

   assert_eq!(
    lineup_queue.reschedable_txs.len(),
    1
   );

   println!("tx_on_test{:?}",chain_trnasaction.chain_transaction);
   println!("len{:?}",transaction_on_thread.trnasaction_on_thread);

   chain_trnasaction.process_all_transaction_from_thread_1(transaction_on_thread.clone() , 1);
   chain_trnasaction.process_all_transaction_from_thread_1(transaction_on_thread.clone() , 2);

   println!("thread_load_counter {:?}" , thread_load_counter.load_counter)
   

}