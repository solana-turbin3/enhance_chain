use actix_web::web::get;
use solana_sdk::pubkey::Pubkey;

#[derive(Debug,Clone)]
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
    pub account_keys: Pubkey,
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

impl InstructionAccount {
    pub fn create_native_instruction_account_for_transaction(&mut self , index_in_transaction : usize,is_signer:bool,is_writeable:bool) -> Self {
        Self {
            index_in_transaction,
            index_in_caller : 0,
            index_in_callee : None,
            is_signer,
            is_writeable

        }
    }

    pub fn create_main_instruction_account_for_transaction(&mut self, index_in_transaction : usize , is_signer:bool , is_writeable:bool , context_stack_height : usize) -> Self {
        Self {
            index_in_transaction,
            index_in_caller : context_stack_height,
            index_in_callee : Some(0),
            is_signer,
            is_writeable
        }
    }
}

impl Default for InstructionContext {
    fn default() -> Self {
        Self { 
            instruction_accounts: Vec::new() 
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

    pub fn get_native_ins_account(&mut self , index_in_transaction : usize) -> Option<&InstructionAccount> {
        for native_instruction in &self.instruction_accounts {
            if native_instruction.index_in_callee == None && native_instruction.index_in_transaction == index_in_transaction {
                return Some(native_instruction);
            }
        }
        None
    }
}

impl TransactionContext {
    
    pub fn check_for_accounts_permission_previlage_mismatch(&mut self,instruction_context : &mut  InstructionContext) {
        let instruction_context_vec = &mut self.instruction_trace;
        for (index , instruction_accounts) in instruction_context_vec.iter().enumerate() {
            if let Some(my_instruction) = instruction_accounts.instruction_accounts.get(index) {
                let native_instruction = InstructionContext::get_native_ins_account( instruction_context, index);
                if native_instruction.unwrap().is_writeable != my_instruction.is_writeable {
                    panic!("Writeable previlage esclated")
                }
                if native_instruction.unwrap().is_signer != my_instruction.is_signer {
                    panic!("Signer previlage esclated")
                }

            }
        } 
    }
}