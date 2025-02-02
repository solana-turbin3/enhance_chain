use crate::pool;

use super::pool::{ChainPoolsInfo, TxFeePayerPool, TxFeePayerPools};

#[test]
fn test_create_new_pool() {
    let mut chain_pool_info = ChainPoolsInfo {
        num_of_active_fee_payer_pools : 0, total_global_balance : 0
    };

    let mut pools = TxFeePayerPools {
        pools : Vec::new()
    };

    TxFeePayerPool::create_new_pool(
        &mut chain_pool_info,
        &mut pools
    );

    assert_eq!(chain_pool_info.num_of_active_fee_payer_pools,1);
    assert_eq!(pools.pools.len(),1)
}