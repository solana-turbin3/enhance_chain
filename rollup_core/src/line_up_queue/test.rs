use solana_sdk::transaction::Transaction;

use super::line_up_queue::LineUpQueue;

#[test]

fn add_to_line_up() {
    let mut line_up_queue = LineUpQueue::default();
    let txs = Transaction::default();
    line_up_queue.add_to_line_up(1, txs, 6);
    assert_eq!(line_up_queue.lineup_queue.len(),2); 
}

#[test]
fn get_the_lineup() {
    let mut line_up_queue = LineUpQueue::default();
    let txs = Transaction::default();
    line_up_queue.add_to_line_up(1, txs, 6);
    assert_eq!(line_up_queue.lineup_queue.len(),2); 

    let line_up = line_up_queue.get_the_line_up();
    println!("{:?}", line_up)
}

#[test]
#[should_panic(expected="Lineup is not full")]
fn priority_sort_transaction() {
    let mut line_up_queue = LineUpQueue::default();
    let txs = Transaction::default();
    line_up_queue.add_to_line_up(1, txs, 6);
    assert_eq!(line_up_queue.lineup_queue.len(),2);


    let sorted_lineup = line_up_queue.sort_linup_queue_according_to_priority();
    println!("{:?}",sorted_lineup);
    assert_eq!(sorted_lineup.lineup_queue.len(),2);
    assert_eq!(sorted_lineup.lineup_queue[0].priority,6)
}

#[test]
fn clear_lineup_queue() {
    let mut lineup_queue = LineUpQueue::default();
    lineup_queue.clear_lineup_queue_for_next_batch();
    assert_eq!(lineup_queue.lineup_queue.len(),0)
}