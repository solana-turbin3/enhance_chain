use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};

use crate::{chain_entrypoint::tx_entrypoint::TransactionsOnThread, line_up_queue::line_up_queue::{AccountInvolvedInTransaction, LineUpQueue}, processor::transaction::ForTransferTransaction, scheduler::read_write_locks::ThreadAwareLocks};

use super::tx_entrypoint::ChainTransaction;

#[test]

fn test_full_flow() {

    let mut chain_trnasaction = ChainTransaction::default();

    let w_account = Keypair::new().pubkey();
    let r_account = Keypair::new().pubkey();
    let from = Keypair::new();
    let to = Keypair::new();
    let transaction_metadata = ForTransferTransaction {
        amount : 10_000_000,
        mint : None,
        from : from.pubkey(),
        to : to.pubkey()
    };

    let transaction_accounts = AccountInvolvedInTransaction {
        is_writeable_accounts : vec![w_account],
        non_writeable_accounts : vec![r_account]
    };

    let mut lineup_queue = LineUpQueue::default();
    let mut thread_aware_locks = ThreadAwareLocks::new(4);
    let mut transaction_on_thread = TransactionsOnThread::default();

    chain_trnasaction.push_new_transaction_to_the_main_queue(&mut lineup_queue, transaction_accounts, transaction_metadata , from,to);

   chain_trnasaction.put_all_the_transaction_in_the_lineup_queue(&mut lineup_queue);

   chain_trnasaction.sort_transaction_in_lineup_queue_by_priority(&mut lineup_queue);

   assert_eq!(
    lineup_queue.lineup_queue.len(),
    1
   );

   chain_trnasaction.take_out_individual_transaction_and_apply_RWlocks(&mut lineup_queue, &mut thread_aware_locks,&mut transaction_on_thread);

   assert_eq!(
    chain_trnasaction.chain_transaction.len(),
    1
   );

   assert_eq!(
    transaction_on_thread.trnasaction_on_thread.len(),
    1
   );

   assert_eq!(
    lineup_queue.reschedable_txs.len(),
    0
   );

   println!("{:?}",chain_trnasaction.chain_transaction);
   println!("{:?}",transaction_on_thread.trnasaction_on_thread);

   chain_trnasaction.process_all_transaction_from_thread_1(transaction_on_thread);

}