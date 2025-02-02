use solana_sdk::{signature::Keypair, signer::keypair};

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

#[test]
fn test_get_specific_pool() {
    let pool = TxFeePayerPool { pool_id: 1, balance: 100, key: Keypair::new() };
    let mut pools = TxFeePayerPools { pools: vec![pool] };
    
    let retrieved_pool = pools.get_specific_pool(1);
    println!("Retrieved pool: {:?}", retrieved_pool);
    
    assert!(retrieved_pool.is_some());
    assert_eq!(retrieved_pool.unwrap().pool_id, 1);
}

#[test]
fn add_funds_in_a_specific_pool() {
    let key_pair1 = Keypair::new();
    let key_pair_2 = Keypair::new();
    let pool = TxFeePayerPool {
        pool_id : 1,
        balance : 100,
        key : key_pair1
    };

    let mut pools = TxFeePayerPools {
        pools : vec![pool]
    };
    pools.pools.push(
        TxFeePayerPool{
            pool_id : 2,
            balance : 100,
            key : key_pair_2
        }
    );
    pools.add_funds_in_a_specific_pool(2, 100);
    assert_eq!(pools.pools.len() , 2);
    assert_eq!(pools.pools[0].balance, 100);
    assert_eq!(pools.pools[1].balance, 200);
}

#[test]
fn deactive_pool() {
    let key_pair1 = Keypair::new();
    let key_pair2 = Keypair::new();
    let pool = TxFeePayerPool {
        pool_id : 1,
        balance : 0,
        key : key_pair1
    };
    let mut pools = TxFeePayerPools {
        pools : vec![pool]
    };
    pools.pools.push(
        TxFeePayerPool{
            pool_id : 2,
            balance : 100,
            key : key_pair2
        }
    );
    assert_eq!(pools.pools.len(),2);
    pools.deactive_pool(1);
    assert_eq!(pools.pools.len(),1);
    assert_eq!(pools.pools[0].pool_id,2) 
}

#[test]
#[should_panic(expected="Pool balance is not empty")]
fn deactive_pool_with_non_zero_balance() {
    let key_pair1 = Keypair::new();
    let key_pair2 = Keypair::new();
    let pool = TxFeePayerPool {
        pool_id : 1,
        balance : 100,
        key : key_pair1
    };
    let mut pools = TxFeePayerPools {
        pools : vec![pool]
    };
    pools.pools.push(
        TxFeePayerPool{
            pool_id : 2,
            balance : 100,
            key : key_pair2
        }
    );
    assert_eq!(pools.pools.len(),2);
    pools.deactive_pool(1);
    assert_eq!(pools.pools.len(),1);
    assert_eq!(pools.pools[0].pool_id,2) 
}

#[test]
fn test_global_balance() {
    let mut pools = TxFeePayerPools { pools: vec![
        TxFeePayerPool { pool_id: 1, balance: 100, key: Keypair::new() },
        TxFeePayerPool { pool_id: 2, balance: 200, key: Keypair::new() }
    ]};
    
    let total_balance = pools.global_balance();
    println!("Total global balance: {}", total_balance);
    
    assert_eq!(total_balance, 300);
}