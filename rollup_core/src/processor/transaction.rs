use {
    solana_sdk::{
        instruction::Instruction as SolanaInstruction, pubkey::Pubkey, signature::Keypair, system_instruction, transaction::{
            SanitizedTransaction as SolanaSanitizedTransaction, Transaction as SolanaTransaction,
        }
    },
    spl_associated_token_account::get_associated_token_address,
    std::collections::HashSet,
};

#[derive(Clone,Debug)]
pub struct TransactionMetadata {
    pub mint: Option<Pubkey>,
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
}


impl From<&TransactionMetadata> for SolanaInstruction {
    fn from(value: &TransactionMetadata) -> Self {
        let TransactionMetadata {
            mint,
            from,
            to,
            amount,

        } = value;
        if let Some(mint) = mint {
            let source_pubkey = get_associated_token_address(from, mint);
            let destination_pubkey = get_associated_token_address(to, mint);
            return spl_token::instruction::transfer(
                &spl_token::id(),
                &source_pubkey,
                &destination_pubkey,
                from,
                &[],
                *amount,
            )
            .unwrap();
        }
        system_instruction::transfer(from, to, *amount)
    }
}

impl From<&TransactionMetadata> for SolanaTransaction {
    fn from(value: &TransactionMetadata) -> Self {
        SolanaTransaction::new_with_payer(&[SolanaInstruction::from(value)], Some(&value.from))
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

