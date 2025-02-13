use {
    serde::de, solana_sdk::{
        instruction::Instruction as SolanaInstruction, pubkey::Pubkey, signature::Keypair, system_instruction, transaction::{
            SanitizedTransaction as SolanaSanitizedTransaction, Transaction as SolanaTransaction,
        }
    }, spl_associated_token_account::get_associated_token_address, std::collections::HashSet
};

#[derive(Clone,Debug)]
pub enum TransactionItem {
    Pubkey(Pubkey),
    Amount(u64)
}

#[derive(Clone,Debug,Hash)]
pub enum TransactionType {
    Transfer = 0,
   // InitAccount = 1
}

#[derive(Clone,Debug,Hash)]
pub struct TransactionMetadata {
    pub txs_type : TransactionType,
    pub keys : Vec<Option<Pubkey>>,
    pub args : Vec<u64>
}

impl From<&TransactionMetadata> for SolanaInstruction {
    fn from(value: &TransactionMetadata) -> Self {
        // Metadata structure for transfer
        //     payer //0
        //     mint, //1
        //     from, //2
        //     to, //3
        //     amount, //0 in args
        match value.txs_type {
            TransactionType::Transfer => {
                if let Some(_mint) = value.keys[1] {
                    let source_pubkey = get_associated_token_address(&value.keys[2].unwrap(), &value.keys[1].unwrap());
                    let destination_pubkey = get_associated_token_address(&value.keys[3].unwrap(), &value.keys[1].unwrap());
                    return spl_token::instruction::transfer(
                        &spl_token::id(),
                        &source_pubkey,
                        &destination_pubkey,
                        &value.keys[2].unwrap(),
                        &[],
                        value.args[0],
                    )
                    .unwrap();
                }
                system_instruction::transfer(&value.keys[2].unwrap(), &value.keys[3].unwrap(), value.args[0])
            }
        }
    }
}

impl From<&TransactionMetadata> for SolanaTransaction {
    fn from(value: &TransactionMetadata) -> Self {
        SolanaTransaction::new_with_payer(&[SolanaInstruction::from(value)], Some(&value.keys[0].unwrap()))
    }
}

impl From<&TransactionMetadata> for SolanaSanitizedTransaction {
    fn from(value: &TransactionMetadata) -> Self {
        SolanaSanitizedTransaction::try_from_legacy_transaction(
            SolanaTransaction::from(value),
            &HashSet::new(),
        )
        .unwrap()
    }
}

/// Create a batch of Solana transactions, for the Solana SVM's transaction
/// processor, from a batch of PayTube instructions.
pub fn create_svm_transactions(
    paytube_transactions: &[TransactionMetadata],
) -> Vec<SolanaSanitizedTransaction> {
    paytube_transactions
        .iter()
        .map(SolanaSanitizedTransaction::from)
        .collect()
}

