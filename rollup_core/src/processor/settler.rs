use {
    super::transaction::TransactionMetadata, solana_client::{rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig}, solana_sdk::{
        commitment_config::CommitmentConfig, instruction::Instruction as SolanaInstruction,
        pubkey::Pubkey, signature::Keypair, signer::Signer, system_instruction,
        transaction::Transaction as SolanaTransaction,
    }, solana_svm::{
        transaction_processing_result::TransactionProcessingResultExtensions,
        transaction_processor::LoadAndExecuteSanitizedTransactionsOutput,
    }, spl_associated_token_account::get_associated_token_address, std::collections::HashMap
};

// #[derive(PartialEq, Eq, Hash)]
// struct LedgerKey {
//     mint: Option<Pubkey>,
//     keys: [Pubkey; 2],
// }

// struct Ledger {
//     ledger: HashMap<LedgerKey, i128>,
// }

//impl Ledger {
    // fn new(
    //     paytube_transactions: &[TransactionMetadata],
    //     svm_output: LoadAndExecuteSanitizedTransactionsOutput,
    // ) -> Self {
    //     let mut ledger: HashMap<LedgerKey, i128> = HashMap::new();
    //     paytube_transactions
    //         .iter()
    //         .zip(svm_output.processing_results)
    //         .for_each(|(transaction, result)| {
    //             // Only append to the ledger if the PayTube transaction was
    //             // successful.
    //             if result.was_processed_with_successful_result() {
    //                 let mint = transaction.mint;
    //                 let mut keys = [transaction.from, transaction.to];
    //                 keys.sort();
    //                 let amount = if keys.iter().position(|k| k.eq(&transaction.from)).unwrap() == 0
    //                 {
    //                     transaction.amount as i128
    //                 } else {
    //                     (transaction.amount as i128)
    //                         .checked_neg()
    //                         .unwrap_or_default()
    //                 };
    //                 ledger
    //                     .entry(LedgerKey { mint, keys })
    //                     .and_modify(|e| *e = e.checked_add(amount).unwrap())
    //                     .or_insert(amount);
    //             }
    //         });
    //     Self { ledger }
    // }


//}

const CHUNK_SIZE: usize = 10;

/// PayTube final transaction settler.
pub struct PayTubeSettler<'a> {
    instructions: Vec<SolanaInstruction>,
    keys: &'a [Keypair],
    rpc_client: &'a RpcClient,
}

impl<'a> PayTubeSettler<'a> {
    /// Create a new instance of a `PayTubeSettler` by tallying up all
    /// transfers into a ledger.
    pub fn new(
        rpc_client: &'a RpcClient,
        transactions: &[TransactionMetadata],
        svm_output: LoadAndExecuteSanitizedTransactionsOutput,
        keys: &'a [Keypair],
    ) -> Self {
        // Build the ledger from the processed PayTube transactions.
        //let ledger = self(paytube_transactions, svm_output);

        // Build the Solana instructions from the ledger.
        let instructions = generate_base_chain_instructions(transactions);
        println!("LLinstructions{:?}", instructions);
        Self {
            instructions,
            keys,
            rpc_client,
        }
    }

    /// Count how many settlement transactions are estimated to be required.
    pub(crate) fn num_transactions(&self) -> usize {
        self.instructions.len().div_ceil(CHUNK_SIZE)
    }

    /// Settle the payment channel results to the Solana blockchain.
    pub fn process_settle(&self) {
        let recent_blockhash = self.rpc_client.get_latest_blockhash().unwrap();
        self.instructions.chunks(CHUNK_SIZE).for_each(|chunk| {
            let transaction = SolanaTransaction::new_signed_with_payer(
                chunk,
                Some(&self.keys[0].pubkey()),
                self.keys,
                recent_blockhash,
            );
            self.rpc_client
                .send_and_confirm_transaction_with_spinner_and_config(
                    &transaction,
                    CommitmentConfig::processed(),
                    RpcSendTransactionConfig {
                        skip_preflight: true,
                        ..Default::default()
                    },
                )
                .unwrap();

            // println!("{:?}", transaction.signatures)
        });
    }
}


pub fn generate_base_chain_instructions(transactions: &[TransactionMetadata]) -> Vec<SolanaInstruction> {
        transactions
            .iter()
            .map(|transaction|{
               
                   
                if let Some(mint) = transaction.keys[1] {
                     // Metadata structure for transfer
                    //     payer //0
                    //     mint, //1
                    //     from, //2
                    //     to, //3
                    //     amount, //0 in args
                    let source_pubkey = get_associated_token_address(&transaction.keys[2].unwrap(), &transaction.keys[1].unwrap());
                    let destination_pubkey = get_associated_token_address(&transaction.keys[3].unwrap(), &transaction.keys[1].unwrap());
                    return spl_token::instruction::transfer(
                        &spl_token::id(),
                        &source_pubkey,
                        &destination_pubkey,
                        &transaction.keys[2].unwrap(),
                        &[],
                        transaction.args[0],
                    )
                    .unwrap();
                }
                system_instruction::transfer(&transaction.keys[2].unwrap(), &transaction.keys[3].unwrap(), transaction.args[0])
                  

                
            })
            .collect::<Vec<_>>()
    
}