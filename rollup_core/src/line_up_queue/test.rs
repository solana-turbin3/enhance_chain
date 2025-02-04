use solana_sdk::transaction::Transaction;

use super::line_up_queue::{LineUpQueue, TransactionsInQueue};

// #[test]

// fn add_to_line_up() {
//     let mut line_up_queue = LineUpQueue::default();
//     let txs = Transaction::default();
//     line_up_queue.add_to_line_up(1, txs, 6);
//     assert_eq!(line_up_queue.lineup_queue.len(),2); 
// }

// #[test]
// fn get_the_lineup() {
//     let mut line_up_queue = LineUpQueue::default();
//     let txs = Transaction::default();
//     line_up_queue.add_to_line_up(1, txs, 6);
//     assert_eq!(line_up_queue.lineup_queue.len(),2); 

//     let line_up = line_up_queue.get_the_line_up();
//     println!("{:?}", line_up)
// }

// #[test]
// #[should_panic(expected="Lineup is not full")]
// fn priority_sort_transaction() {
//     let mut line_up_queue = LineUpQueue::default();
//     let txs = Transaction::default();
//     line_up_queue.add_to_line_up(1, txs, 6);
//     assert_eq!(line_up_queue.lineup_queue.len(),2);


//     let sorted_lineup = line_up_queue.sort_linup_queue_according_to_priority();
//     println!("{:?}",sorted_lineup);
//     assert_eq!(sorted_lineup.lineup_queue.len(),2);
//     assert_eq!(sorted_lineup.lineup_queue[0].priority,6)
// }

#[test]
fn clear_lineup_queue() {
    let mut lineup_queue = LineUpQueue::default();
    lineup_queue.clear_lineup_queue_for_next_batch();
    assert_eq!(lineup_queue.lineup_queue.len(),0)
}

#[test]
fn full_flow() {
    let mut line_up_queue = LineUpQueue::default();

    for i in 0..15 {
        line_up_queue.main_queue.push(
            TransactionsInQueue {
                id : i,
                txs : Transaction::default(),
                priority : i
            }
        );
    }
    
    line_up_queue.add_to_line_up();
    assert_eq!(
        line_up_queue.main_queue.len(),
        5
    );
    assert_eq!(
        line_up_queue.lineup_queue.len(),
        10
    );
    println!("{:?}",line_up_queue.main_queue.len());
    println!("{:?}",line_up_queue.lineup_queue.len());

    for i in 0..10 {
        line_up_queue.add_transaction_to_non_rescheduable_container(i, i, Transaction::default());
    }

    assert_eq!(
        line_up_queue.reschedable_txs.len(),
        5
    );
    println!("{:?}",line_up_queue.reschedable_txs.len());

    assert_eq!(
        line_up_queue.main_queue.len(),
        10
    );
    println!("{:?}",line_up_queue.main_queue.len());
    println!("{:?}",line_up_queue.lineup_queue.len());

}