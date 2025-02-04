use anyhow::Ok;
use solana_sdk::transaction::Transaction;

const TOTAL_LINUP_BUDGET : u32  = 10;
const PER_LINEUP_BUDGET : u32 = 1;

const TOTAL_RESCHEDUABLE_BUDGET : u32  = 5;
const PER_RESCHEDUABLE_BUDGET : u32 = 1;

#[derive(Debug,Clone)]
pub struct TransactionsInQueue {
    pub id : u64,
    pub txs : Transaction,
    pub priority : u64,
}

#[derive(Debug)]
pub struct LineUpQueue {
    pub lineup_budget_counter : u32,
    pub rescheduable_budget : u32,
    pub lineup_queue : Vec<TransactionsInQueue>,
    pub reschedable_txs : Vec<TransactionsInQueue>,
    pub main_queue : Vec<TransactionsInQueue>
}

impl Default for LineUpQueue {
    fn default() -> Self {
        LineUpQueue {
            lineup_budget_counter: 0, 
            rescheduable_budget : 0,      
            lineup_queue: {
                let mut queue = Vec::new();
                queue
            },
            reschedable_txs : Vec::new(),
            main_queue : Vec::new() 
        }
    }
}

impl LineUpQueue {

    pub fn add_to_main_tx_queue(&mut self,id:u64,txs:Transaction,priority:u64) {
        self.main_queue.push(
            TransactionsInQueue {
                id,
                txs,
                priority
            }
        );
    }
    
    //IMP- clone()
    pub fn add_to_line_up(&mut self) {
        let reschedable_txs_clone = self.reschedable_txs.clone();
        for rescheduable_txs in reschedable_txs_clone {
            self.add_transaction_to_non_rescheduable_container(
                rescheduable_txs.id,
                rescheduable_txs.priority,
                rescheduable_txs.txs,
            );
        }
        
        let mut i = 0;
        while i < self.main_queue.len() {
            if self.lineup_budget_counter < TOTAL_LINUP_BUDGET {
                let transaction = self.main_queue.remove(i); // Removes transaction from main_queue
                self.lineup_queue.push(TransactionsInQueue {
                    id: transaction.id,
                    txs: transaction.txs,
                    priority: transaction.priority,
                });
                self.lineup_budget_counter += PER_LINEUP_BUDGET;
            } else {
                break;
            }
        }
    }
    
    
    // Result -> Ok()
    //self
    pub fn sort_linup_queue_according_to_priority(&mut self) -> &mut Self{
        if self.lineup_queue.len() <10 {
            panic!("Lineup is not full");
        } else {
            self.lineup_queue.sort_by(|a, b| b.priority.cmp(&a.priority)); 
            self   
        }
    }

    pub fn get_the_line_up(&mut self) -> &Vec<TransactionsInQueue> {
        &self.lineup_queue
    }

    pub fn clear_lineup_queue_for_next_batch(&mut self) {
        // TODO/f
        // cleanup when the batch is sent
        self.lineup_queue.clear();
    }

    pub fn add_transaction_to_non_rescheduable_container(
        &mut self,
        id : u64,
        priority : u64,
        txs : Transaction
    ) {
        if self.rescheduable_budget < TOTAL_RESCHEDUABLE_BUDGET {
            self.reschedable_txs.push(
                TransactionsInQueue {
                    id,
                    txs,
                    priority,
                }
            );
            self.rescheduable_budget += PER_RESCHEDUABLE_BUDGET
        } else {
            self.add_to_main_tx_queue(id, txs, priority);
        }
    }

}

