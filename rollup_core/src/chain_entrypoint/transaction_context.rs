use actix_web::web::get;
use solana_sdk::pubkey::{self, Pubkey};

#[derive(Debug,Clone,Copy)]
pub struct InstructionAccount {
    pub index_in_transaction : usize,
    pub index_in_caller : usize,
    pub index_in_callee : Option<usize>,
    pub is_writeable : bool,
    pub is_signer : bool
}

#[derive(Debug,Clone)]
pub struct InstructionContext {
    //pub nesting_level: usize,
    // instruction_accounts_lamport_sum: u128,
    // program_accounts: Vec<IndexOfAccount>,
    pub instruction_accounts: Vec<InstructionAccount>,
    // instruction_data:    `Vec<u8>,
}

#[derive(Debug,Clone)]
pub struct TransactionContext {
    pub account_keys: Vec<Pubkey>,
    //pub accounts: Rc<TransactionAccounts>,
    // instruction_stack_capacity: usize,
    // instruction_trace_capacity: usize,
    // instruction_stack: Vec<usize>,
    pub instruction_trace: Vec<InstructionContext>,
    // return_data: TransactionReturnData,
    // accounts_resize_delta: RefCell<i64>,
    // #[cfg(not(target_os = "solana"))]
    // remove_accounts_executable_flag_checks: bool,
    // #[cfg(not(target_os = "solana"))]
    // rent: Rent,
    // /// Useful for debugging to filter by or to look it up on the explorer
    // #[cfg(all(
    //     not(target_os = "solana"),
    //     feature = "debug-signature",
    //     debug_assertions
    // ))]
    // signature: Signature,
}

impl Default for InstructionContext {
    fn default() -> Self {
        Self { 
            instruction_accounts: Vec::new() 
        }
    }
}

impl Default for TransactionContext {
    fn default() -> Self {
        Self {
            account_keys : Vec::new(),
            instruction_trace : Vec::new()
        }
    }
}

impl InstructionContext {

    pub fn add_instruction_context(&mut self , instruction_account : InstructionAccount) {
        self.instruction_accounts.push(instruction_account);
    }

    pub fn get_context_stack_height(&mut self) -> usize {
        return self.instruction_accounts.len();
    }

    pub fn create_native_instruction_account_for_transaction(&mut self , index_in_transaction : usize,is_signer:bool,is_writeable:bool) {
        let instruction_account = InstructionAccount {
            index_in_transaction,
            index_in_caller : 0,
            index_in_callee : None,
            is_signer,
            is_writeable

        };
        self.add_instruction_context(instruction_account);
    }

    pub fn create_main_instruction_account_for_transaction(&mut self, index_in_transaction : usize , is_signer:bool , is_writeable:bool , context_stack_height : usize) {
        let instruction_account = InstructionAccount {
            index_in_transaction,
            index_in_caller : context_stack_height,
            index_in_callee : Some(0),
            is_signer,
            is_writeable
        };
        self.add_instruction_context(instruction_account);
    }


}

impl TransactionContext {

    pub fn fill_accounts(&mut self,account_keys : Vec<Pubkey>) {
        self.account_keys = account_keys
    }

    // normalize
    //TODO://

    pub fn create_native_and_main_ins_account(&mut self, account_keys : Vec<Pubkey> , instruction_context :  &mut InstructionContext) {
        let mut duplicate_indicies : Vec<usize> = Vec::new();

        for account in account_keys {
            let index_in_transaction = self.get_index_of_transaction(&account);
            let stack_height = instruction_context.get_context_stack_height();
            if duplicate_indicies.contains(&index_in_transaction) {
                instruction_context.create_main_instruction_account_for_transaction(index_in_transaction, false, true, stack_height);
            } else {
                instruction_context.create_native_instruction_account_for_transaction(index_in_transaction, true, true);
                duplicate_indicies.push(index_in_transaction);
            }
        }

    }

    pub fn main(&mut self , instruction_context : &mut InstructionContext ,transaction_accounts : Vec<Pubkey>) {
        self.create_native_and_main_ins_account(transaction_accounts.clone(), instruction_context);
        for account in transaction_accounts {
            self.check_for_accounts_permission_previlage_mismatch(instruction_context.clone(), account);
        }
    }

    pub fn get_index_of_transaction(&mut self , account : &Pubkey) -> usize {
        let index = self.account_keys.iter().position(|&key| &key == account).unwrap();
        index
    }

    pub fn check_for_accounts_permission_previlage_mismatch(&mut self,instruction_context :  InstructionContext,account:Pubkey) {
        println!("{:?}",instruction_context);

        for instruction_account in instruction_context.instruction_accounts.iter() {
                let native_instruction = self.get_native_ins_account(instruction_context.clone(),account).unwrap();
                println!("native_ins{:?}",native_instruction);
                if native_instruction.is_writeable != instruction_account.is_writeable {
                    panic!("Writeable previlage esclated")
                }
                if native_instruction.is_signer != instruction_account.is_signer {
                    panic!("Signer previlage esclated")
                }
        } 
    }

    pub fn get_native_ins_account(&mut self,instruction_context : InstructionContext , account : Pubkey) -> Option<InstructionAccount> {
        let index_in_trasaction = self.get_index_of_transaction(&account);
        for (_index,native_instruction) in instruction_context.instruction_accounts.iter().enumerate() {
            if native_instruction.index_in_callee == None && native_instruction.index_in_transaction == index_in_trasaction {
                return Some(*native_instruction);
            }
        }
        None
    }

}


mod test {
    use solana_sdk::{signature::Keypair, signer::{keypair, Signer}};

    use crate::chain_entrypoint::transaction_context::TransactionContext;

    use super::{InstructionAccount, InstructionContext};

    #[test]
    fn test_transaction_context_flow() {
        let mut instruction_context = InstructionContext::default();
        let mut transaction_context = TransactionContext::default();

        let kp1 = Keypair::new().pubkey();
        let kp2 = Keypair::new().pubkey();

        let transaction_accounts = vec![
            kp1,
            kp2,
            kp1
        ];

        transaction_context.fill_accounts(transaction_accounts.clone());
        transaction_context.main(&mut instruction_context,transaction_accounts);
    } 
}