use actix_web::web::get;
use solana_sdk::pubkey::Pubkey;

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
    //pub account_keys: Pubkey,
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

    pub fn check_for_accounts_permission_previlage_mismatch(&mut self,instruction_context :  InstructionContext,account_index:usize) {
       // let instruction_context_vec = &mut instruction_context.instruction_accounts;
        // println!("{:?}",self.instruction_trace.len());
        println!("{:?}",instruction_context);
        for (index , instruction_account) in instruction_context.instruction_accounts.iter().enumerate() {
            println!("{:?} {:?}",index,instruction_account);
                let native_instruction = get_native_ins_account(instruction_context.clone() ,account_index).unwrap();
                println!("native_ins{:?}",native_instruction);
                if native_instruction.is_writeable != instruction_account.is_writeable {
                    panic!("Writeable previlage esclated")
                }
                if native_instruction.is_signer != instruction_account.is_signer {
                    panic!("Signer previlage esclated")
                }

            
        } 
    }
}



pub fn get_native_ins_account(instruction_context : InstructionContext , index_in_transaction : usize) -> Option<InstructionAccount> {
    for (index,native_instruction) in instruction_context.instruction_accounts.iter().enumerate() {
        println!("milaab {:?} {:?} {:?}",index,native_instruction.index_in_callee,native_instruction.index_in_transaction);
        if native_instruction.index_in_callee == None && native_instruction.index_in_transaction == index_in_transaction {
            return Some(*native_instruction);
        }
    }
    None
}

mod test {
    use crate::chain_entrypoint::transaction_context::TransactionContext;

    use super::{InstructionAccount, InstructionContext};

    #[test]
    fn test_transaction_context_flow() {
        let mut instruction_context = InstructionContext::default();
        let mut transaction_context = TransactionContext::default();

        let from_native_ins = instruction_context.create_native_instruction_account_for_transaction(
            0,true,true
        );

        // let to_native_ins = instruction_context.create_native_instruction_account_for_transaction(
        //     1,true,true
        // );

        let stack_height = instruction_context.get_context_stack_height();

        let main_from_ins = instruction_context.create_main_instruction_account_for_transaction(0, false, false, stack_height);

        assert_eq!(
            stack_height,
            1
        );

        assert_eq!(
            instruction_context.instruction_accounts.len(),
            2
        );

        // let stack_height = instruction_context.get_context_stack_height();

        // let main_to_ins = instruction_context.create_main_instruction_account_for_transaction(1, true, false, stack_height);

        //println!("{:?}",instruction_context.instruction_accounts);
        transaction_context.check_for_accounts_permission_previlage_mismatch(instruction_context,0);
    } 
}