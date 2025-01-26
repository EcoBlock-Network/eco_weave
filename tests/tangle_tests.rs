use eco_weave::{Tangle, Transaction};
use tokio;

#[test]
fn test_add_transaction() {
    let mut tangle = Tangle::new();
    let tx = Transaction::new("tx1", "Sample transaction");
    assert!(tangle.add_transaction(tx.clone()));
    assert_eq!(tangle.transactions.len(), 1);
    assert_eq!(tangle.transactions["tx1"], tx);
}

#[test]
fn test_duplicate_transaction() {
    let mut tangle = Tangle::new();
    let tx = Transaction::new("tx1", "Duplicate test");
    tangle.add_transaction(tx.clone());
    assert!(!tangle.add_transaction(tx));
}

#[tokio::test]
async fn test_propagate_transaction() {
    let mut tangle = Tangle::new();

    tangle.add_node("node1");
    tangle.add_node("node2");
    tangle.add_node("node3");
    tangle.connect_nodes("node1", "node2");
    tangle.connect_nodes("node2", "node3");

    let tx = Transaction::new("tx1", "Hello, Tangle!");

    let propagated_count = tangle.propagate_transaction(tx.clone(), "node1").await;
    assert_eq!(propagated_count, 3);

    assert!(tangle.transactions.contains_key("tx1"));
    assert_eq!(tangle.transactions.len(), 1);
}

#[tokio::test]
async fn test_propagate_transaction_with_missing_node() {
    let mut tangle = Tangle::new();

    let tx = Transaction::new("tx1", "Hello, Tangle!");

    let propagated_count = tangle.propagate_transaction(tx.clone(), "missing_node").await;

    assert_eq!(propagated_count, 0);
    assert!(tangle.transactions.is_empty());
}

#[test]
fn test_add_valid_transaction() {
    let mut tangle = Tangle::default();
    let tx = Transaction::new("tx1", "Valid payload");
    assert!(tangle.add_transaction(tx));
}

#[test]
fn test_add_duplicate_transaction() {
    let mut tangle = Tangle::default();
    let tx = Transaction::new("tx1", "Valid payload");
    assert!(tangle.add_transaction(tx.clone()));
    assert!(!tangle.add_transaction(tx)); 
}

#[test]
fn test_add_invalid_transaction_empty_id() {
    let mut tangle = Tangle::default();
    let tx = Transaction::new("", "Payload");
    assert!(!tangle.add_transaction(tx)); 
}

#[test]
fn test_add_invalid_transaction_empty_payload() {
    let mut tangle = Tangle::default();
    let tx = Transaction::new("tx2", "");
    assert!(!tangle.add_transaction(tx)); 
}

#[tokio::test]
async fn test_weighted_random_walk() {
    let mut tangle = Tangle::new();

    tangle.add_node("tx1");
    tangle.add_node("tx2");

    tangle.connect_nodes("tx1", "tx2");

    let mut tx1 = Transaction::new("tx1", "Payload");
    let mut tx2 = Transaction::new("tx2", "Payload");
    tx1.weight = 10;
    tx2.weight = 20;

    tangle.add_transaction(tx1);
    tangle.add_transaction(tx2);

    let result = tangle.weighted_random_walk("tx1").await;

    assert!(
        result == Some("tx1".to_string()) || result == Some("tx2".to_string()),
        "Unexpected WRW result: {:?}",
        result
    );
}

#[tokio::test]
async fn test_weighted_random_walk_with_no_neighbors() {
    let mut tangle = Tangle::new();

    tangle.add_node("tx1");

    let tx1 = Transaction::new("tx1", "Payload");
    tangle.add_transaction(tx1);

    let result = tangle.weighted_random_walk("tx1").await;

    assert_eq!(result, Some("tx1".to_string()));
}
