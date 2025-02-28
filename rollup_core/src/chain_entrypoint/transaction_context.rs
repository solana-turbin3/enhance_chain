use std::collections::HashMap;

use actix_web::web::get;
use serde::{Deserialize, Serialize};
use solana_sdk::{pubkey::{self, Pubkey}, reserved_account_keys};

use super::tx_entrypoint::AccountsMeta;

#[derive(Debug,Clone,Copy,Serialize,Deserialize)]
pub struct InstructionAccount {
    pub index_in_transaction : usize,
    pub index_in_caller : usize,
    pub index_in_callee : Option<usize>,
    pub is_writeable : bool,
    pub is_signer : bool
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct InstructionContext {
    //pub nesting_level: usize,
    // instruction_accounts_lamport_sum: u128,
    // program_accounts: Vec<IndexOfAccount>,
    pub instruction_accounts: Vec<InstructionAccount>,
    // instruction_data:    `Vec<u8>,
}

#[derive(Debug,Clone)]
pub struct TransactionContext {
    pub account_keys: Vec<AccountsMeta>,
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

    pub fn fill_accounts(&mut self,account_keys : Vec<AccountsMeta>) {
        self.account_keys = account_keys
    }

    pub fn create_native_and_main_ins_account(&mut self, account_keys : Vec<AccountsMeta> , instruction_context :  &mut InstructionContext) {
        let mut duplicate_account : HashMap<Pubkey,usize> = HashMap::new();

        for (index,account) in account_keys.clone().iter().enumerate() {
            let index_in_transaction = self.get_index_of_transaction(&account.key);
            let stack_height = instruction_context.get_context_stack_height();
            println!("index {}",index);
            if duplicate_account.contains_key(&account.key) {
                let index = duplicate_account.get(&account.key).unwrap();
                let mut instruction_account = *instruction_context.instruction_accounts.get(*index).unwrap();
               
                //normalize account preveliges
                instruction_account.is_writeable |= account.is_writeable;
                instruction_account.is_signer |= account.is_signer;


            } else {
                instruction_context.create_native_instruction_account_for_transaction(index_in_transaction, true, true);
                instruction_context.create_main_instruction_account_for_transaction(index_in_transaction, account_keys[index].is_signer, account_keys[index].is_writeable, stack_height);
                duplicate_account.insert(account.key,instruction_context.instruction_accounts.len()-1);
            }
        }

        println!("dup {:?}" , duplicate_account);

    }

    pub fn main(&mut self , instruction_context : &mut InstructionContext ,transaction_accounts : Vec<AccountsMeta>) {
        self.create_native_and_main_ins_account(transaction_accounts.clone(), instruction_context);
        for account in transaction_accounts {
            self.check_for_accounts_permission_previlage_mismatch(instruction_context.clone(), account);
        }
    }

    pub fn get_index_of_transaction(&mut self , account : &Pubkey) -> usize {
        let index = self.account_keys.iter().position(|key| &key.key == account).unwrap();
        index
    }

    pub fn check_for_accounts_permission_previlage_mismatch(&mut self,instruction_context :  InstructionContext,account:AccountsMeta) {
        
        println!("{:?}", instruction_context);
        for instruction_account in instruction_context.instruction_accounts.iter() {
                let native_instruction = self.get_native_ins_account(instruction_context.clone(),account.key).unwrap();
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


pub mod test {
    use solana_sdk::{signature::Keypair, signer::{keypair, Signer}};

    use crate::chain_entrypoint::{transaction_context::TransactionContext, tx_entrypoint::AccountsMeta};

    use super::{InstructionAccount, InstructionContext};

    #[test]
    fn test_transaction_context_flow() {
        let mut instruction_context = InstructionContext::default();
        let mut transaction_context = TransactionContext::default();

        let kp1 = Keypair::new().pubkey();
        let kp2 = Keypair::new().pubkey();

        println!("keypair1 {:?}",kp1);
        println!("keypair2 {:?}",kp2);

        let transaction_account_meta = vec![
            AccountsMeta::create_new_meta_with_signer(kp1, true),
            AccountsMeta::create_new_meta_with_signer(kp2, true),
            AccountsMeta::create_new_meta_with_signer(kp2, false)
        ];

        transaction_context.fill_accounts(transaction_account_meta.clone());
        transaction_context.main(&mut instruction_context,transaction_account_meta);
    } 

    #[test]
    #[should_panic(expected = "Writeable previlage esclated")]
    fn test_transaction_context_flow_with_wrong_preveliges() {
        let mut instruction_context = InstructionContext::default();
        let mut transaction_context = TransactionContext::default();

        let kp1 = Keypair::new().pubkey();
        let kp2 = Keypair::new().pubkey();

        println!("keypair1 {:?}",kp1);
        println!("keypair2 {:?}",kp2);

        let transaction_account_meta = vec![
            AccountsMeta::create_new_meta_with_signer(kp1, true),
            AccountsMeta::create_new_meta_with_signer(kp2, false),
            AccountsMeta::create_new_meta_with_signer(kp2, false)
        ];

        transaction_context.fill_accounts(transaction_account_meta.clone());
        transaction_context.main(&mut instruction_context,transaction_account_meta);
    } 
}