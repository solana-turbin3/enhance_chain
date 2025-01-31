use solana_sdk::transaction::Transaction;

const TOTAL_LINUP_BUDGET : u32  = 10;
const PER_LINEUP_BUDGET : u32 = 1;

pub struct TransactionsInQueue {
    pub id : u64,
    pub txs : Transaction,
    pub priority : u64,
}

pub struct LineUpQueue {
    pub budget_counter : u32,
    pub lineup_queue : Vec<TransactionsInQueue>
}

impl Default for LineUpQueue {
    fn default() -> Self {
        LineUpQueue {
            budget_counter: 0,       
            lineup_queue: Vec::new(),
        }
    }
}

impl LineUpQueue {

    pub fn get_the_linee_up(&mut self) -> &Vec<TransactionsInQueue> {
        &self.lineup_queue
    }

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
    
    pub fn sort_linup_queue_according_to_priority(&mut self){
        self.lineup_queue.sort_by(|a, b| a.priority.cmp(&b.priority));
    }

    pub fn clear_lineup_queue_for_next_batch(&mut self) {
        self.lineup_queue.clear();
    }

}

