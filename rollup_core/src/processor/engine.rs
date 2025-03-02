use super::loader::PayTubeAccountLoader;
use crate::processor::settler::PayTubeSettler;
use crate::processor::transaction::TransactionMetadata;

use {
    crate::processor::processor::{
        create_transaction_batch_processor, get_transaction_check_results, PayTubeForkGraph,
    },
    crate::processor::transaction::create_svm_transactions,
    solana_client::rpc_client::RpcClient,
    solana_compute_budget::compute_budget::ComputeBudget,
    solana_sdk::{
        feature_set::FeatureSet, fee::FeeStructure, hash::Hash, rent_collector::RentCollector,
        signature::Keypair,
    },
    solana_svm::transaction_processor::{
        TransactionProcessingConfig, TransactionProcessingEnvironment,
    },
    std::sync::{Arc, RwLock},
};

/// A PayTube channel instance.
///
/// Facilitates native SOL or SPL token transfers amongst various channel
/// participants, settling the final changes in balances to the base chain.
pub struct PayTubeChannel {
    /// I think you know why this is a bad idea...
    keys: Vec<Keypair>,
    rpc_client: RpcClient,
}

impl PayTubeChannel {
    pub fn new(keys: Vec<Keypair>, rpc_client: RpcClient) -> Self {
        Self { keys, rpc_client }
    }

    /// The PayTube API. Processes a batch of PayTube transactions.
    ///
    /// Obviously this is a very simple implementation, but one could imagine
    /// a more complex service that employs custom functionality, such as:
    ///
    /// * Increased throughput for individual P2P transfers.
    /// * Custom Solana transaction ordering (e.g. MEV).
    ///
    /// The general scaffold of the PayTube API would remain the same.
    pub fn process_paytube_transfers(&self, transactions: &[TransactionMetadata]) {
        // PayTube default configs.
        crate::processor::log::setup_solana_logging();
        //crate::processor::log::creating_paytube_channel();
        let compute_budget = ComputeBudget::default();
        let feature_set = FeatureSet::all_enabled();
        let fee_structure = FeeStructure::default();
        let lamports_per_signature = fee_structure.lamports_per_signature;
        let rent_collector = RentCollector::default();

        // PayTube loader/callback implementation.
        let account_loader = PayTubeAccountLoader::new(&self.rpc_client);

        // Solana SVM transaction batch processor.
        let fork_graph = Arc::new(RwLock::new(PayTubeForkGraph {}));
        let processor = create_transaction_batch_processor(
            &account_loader,
            &feature_set,
            &compute_budget,
            Arc::clone(&fork_graph),
        );

        // The PayTube transaction processing runtime environment.
        let processing_environment = TransactionProcessingEnvironment {
            blockhash: Hash::default(),
            epoch_total_stake: None,
            epoch_vote_accounts: None,
            feature_set: Arc::new(feature_set),
            fee_structure: Some(&fee_structure),
            lamports_per_signature,
            rent_collector: Some(&rent_collector),
        };

        // The PayTube transaction processing config for Solana SVM.
        let processing_config = TransactionProcessingConfig {
            compute_budget: Some(compute_budget),
            ..Default::default()
        };

        // 1. Convert to an SVM transaction batch.
        let svm_transactions = create_svm_transactions(transactions);

        // 2. Process transactions with the SVM API.
        crate::processor::log::processing_transactions(svm_transactions.len());
        let results = processor.load_and_execute_sanitized_transactions(
            &account_loader,
            &svm_transactions,
            get_transaction_check_results(svm_transactions.len(), lamports_per_signature),
            &processing_environment,
            &processing_config,
        );

        // 3. Convert results into a final ledger using a `PayTubeSettler`.
        let settler = PayTubeSettler::new(&self.rpc_client, transactions, results, &self.keys);
        crate::processor::log::settling_to_base_chain(settler.num_transactions());
        // 4. Submit to the Solana base chain.
        settler.process_settle();
        crate::processor::log::channel_closed();
    }
}
