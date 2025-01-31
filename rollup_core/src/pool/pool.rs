use solana_sdk::signature::Keypair;

pub struct ChainPoolsInfo {
    num_of_active_fee_payer_pools : u64,
    total_global_balance : u64
}

pub struct TxFeePayerPool {
    pool_id: u64,
    balance: u64,
    key: Keypair,
}

pub struct TxFeePayerPools {
    pools: Vec<TxFeePayerPool>,
}

impl ChainPoolsInfo {
    fn update_info(&mut self, new_added_balance:u64) {
        self.num_of_active_fee_payer_pools +=1;
        self.total_global_balance += new_added_balance
    }
}

impl TxFeePayerPool {

    fn create_new_pool(chain_pool_info : &mut ChainPoolsInfo, pools : &mut TxFeePayerPools) {
        let pool_id = chain_pool_info.num_of_active_fee_payer_pools;
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
    fn get_specific_pool(&self, id: u64) -> Option<&TxFeePayerPool> {
        match self.pools.iter().find(|&pool| pool.pool_id == id) {
            Some(pool) => Some(pool),
            None => {
                println!("Error: Pool with ID {} not found!", id);
                None
            }
        }
    }

    //sum
    fn global_balance(&mut self) -> u64 {
        self.pools.iter().map(|pool|pool.balance).sum()
    }

    fn add_new_created_pool_in_global_configs(&mut self,pool:TxFeePayerPool) {
        self.pools.push(pool);
    }

    fn add_funds_in_a_specific_pool(&mut self, id:u64 , new_balance : u64)-> Option<u64>{
       match self.pools.iter().find(|pool|pool.pool_id == id) {
        Some(pool) => {
            let updated_balance = pool.balance + new_balance;
            Some(updated_balance)
        },
        None => {
            println!("Error: Pool with ID {} not found!", id);
            None
        }
        }
    }

    //drain and collect 
    fn deactive_pool(&mut self, id: u64) -> TxFeePayerPools {
        let remaining_pools: Vec<TxFeePayerPool> = self.pools
            .drain(..)
            .filter(|pool| pool.pool_id != id)
            .collect();
    
        TxFeePayerPools { 
            pools: remaining_pools 
        }
    }
}