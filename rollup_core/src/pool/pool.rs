use solana_sdk::signature::Keypair;

pub struct ChainPoolsInfo {
    pub num_of_active_fee_payer_pools : u64,
    pub total_global_balance : u64
}

#[derive(Debug)]
pub struct TxFeePayerPool {
    pub pool_id: u64,
    pub balance: u64,
    pub key: Keypair,
}

pub struct TxFeePayerPools {
    pub pools: Vec<TxFeePayerPool>,
}

impl ChainPoolsInfo {
    pub fn update_info(&mut self, new_added_balance:u64) {
        self.num_of_active_fee_payer_pools +=1;
        self.total_global_balance += new_added_balance
    }
}

impl TxFeePayerPool {

    pub fn create_new_pool(chain_pool_info : &mut ChainPoolsInfo, pools : &mut TxFeePayerPools) {
        let pool_id = chain_pool_info.num_of_active_fee_payer_pools | 0 + 1;
        chain_pool_info.num_of_active_fee_payer_pools +=1;
        let new_pool = TxFeePayerPool {
            pool_id : pool_id,
            balance : 0,
            key : Keypair::new()
        };
        pools.add_new_created_pool_in_global_configs(new_pool);
    }
}

impl TxFeePayerPools {

    //& and match
    pub fn get_specific_pool(&mut self, id: u64) -> Option<&TxFeePayerPool> {
        match self.pools.iter().find(|&pool| pool.pool_id == id) {
            Some(pool) => Some(pool),
            None => {
                println!("Error: Pool with ID {} not found!", id);
                None
            }
        }
    }

    //sum
    pub fn global_balance(&mut self) -> u64 {
        self.pools.iter().map(|pool|pool.balance).sum()
    }

    pub fn add_new_created_pool_in_global_configs(&mut self,pool:TxFeePayerPool) {
        self.pools.push(pool);
    }

    pub fn add_funds_in_a_specific_pool(&mut self, id:u64 , new_balance : u64)-> Option<u64>{
       match self.pools.iter_mut().find(|pool|pool.pool_id == id) {
        Some(pool) => {
            let updated_balance = pool.balance + new_balance;
            pool.balance = updated_balance;
            Some(updated_balance)
        },
        None => {
            println!("Error: Pool with ID {} not found!", id);
            None
        }
        }
    }

    //IMP-retain
    pub fn deactive_pool(&mut self, id: u64) {
         // self.pools.retain(|pool| pool.pool_id != id);
         if let Some((index, pool)) = self.pools.iter_mut().enumerate().find(|(_, pool)| pool.pool_id == id) {
            if pool.balance != 0 {
                panic!("Pool balance is not empty")
            } else {
                self.pools.swap_remove(index);
            }
        }
    }
    
}