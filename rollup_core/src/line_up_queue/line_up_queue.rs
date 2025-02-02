use anyhow::Ok;
use solana_sdk::transaction::Transaction;

const TOTAL_LINUP_BUDGET : u32  = 10;
const PER_LINEUP_BUDGET : u32 = 1;

#[derive(Debug)]
pub struct TransactionsInQueue {
    pub id : u64,
    pub txs : Transaction,
    pub priority : u64,
}

#[derive(Debug)]
pub struct LineUpQueue {
    pub budget_counter : u32,
    pub lineup_queue : Vec<TransactionsInQueue>
}

impl Default for LineUpQueue {
    fn default() -> Self {
        LineUpQueue {
            budget_counter: 0,       
            lineup_queue: {
                let mut queue = Vec::new();
                queue.push(TransactionsInQueue {
                    id: 0,
                    txs: Transaction::default(),
                    priority: 1,
                });
                queue
            },
        }
    }
}

impl LineUpQueue {

    pub fn add_to_line_up(&mut self,id:u64,txs:Transaction,priority:u64) {
        if self.budget_counter <= TOTAL_LINUP_BUDGET {
            self.lineup_queue.push(
                TransactionsInQueue{
                    id,
                    txs,
                    priority
                }
            );
            self.budget_counter += PER_LINEUP_BUDGET;
        } else {
            println!("cant add, lineup is full.")
        }
    }
    
    // Result -> Ok()
    // pub fn sort_linup_queue_according_to_priority(&mut self) -> Result<&mut Self, String> {
    //     if self.lineup_queue.len() < 10 {
    //         return Err("Lineup not full".to_string()); // Return an error if lineup is not full
    //     }else {

    //         self.lineup_queue.sort_by(|a, b| b.priority.cmp(&a.priority));
    //         Ok(self) // Return self on success
    //     }
    // }

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

}

