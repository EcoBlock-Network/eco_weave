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



#[tokio::test]
async fn test_weighted_random_walk_complex_graph() {
    let mut tangle = Tangle::new();

    tangle.add_node("tx1");
    tangle.add_node("tx2");
    tangle.add_node("tx3");
    tangle.add_node("tx4");
    tangle.add_node("tx5");


    tangle.connect_nodes("tx1", "tx2");
    tangle.connect_nodes("tx2", "tx3");
    tangle.connect_nodes("tx3", "tx4");
    tangle.connect_nodes("tx4", "tx5");
    tangle.connect_nodes("tx2", "tx4");

    let mut tx1 = Transaction::new("tx1", "Payload1");
    let mut tx2 = Transaction::new("tx2", "Payload2");
    let mut tx3 = Transaction::new("tx3", "Payload3");
    let mut tx4 = Transaction::new("tx4", "Payload4");
    let mut tx5 = Transaction::new("tx5", "Payload5");
    tx1.weight = 5;
    tx2.weight = 15;
    tx3.weight = 10;
    tx4.weight = 20;
    tx5.weight = 1;

    tangle.add_transaction(tx1);
    tangle.add_transaction(tx2);
    tangle.add_transaction(tx3);
    tangle.add_transaction(tx4);
    tangle.add_transaction(tx5);

    let result = tangle.weighted_random_walk("tx1").await;

    assert!(
        result == Some("tx1".to_string())
            || result == Some("tx2".to_string())
            || result == Some("tx3".to_string())
            || result == Some("tx4".to_string())
            || result == Some("tx5".to_string()),
        "Unexpected WRW result: {:?}",
        result
    );
}

#[tokio::test]
async fn test_weighted_random_walk_with_cycles_and_isolated_nodes() {
    let mut tangle = Tangle::new();

    tangle.add_node("tx1");
    tangle.add_node("tx2");
    tangle.add_node("tx3");
    tangle.add_node("tx4");
    tangle.add_node("tx5");
    tangle.add_node("isolated1"); 
    tangle.add_node("isolated2"); 

    tangle.connect_nodes("tx1", "tx2");
    tangle.connect_nodes("tx2", "tx3");
    tangle.connect_nodes("tx3", "tx1");
    tangle.connect_nodes("tx3", "tx4");
    tangle.connect_nodes("tx4", "tx5");
    tangle.connect_nodes("tx5", "tx3");

    let mut tx1 = Transaction::new("tx1", "Payload1");
    let mut tx2 = Transaction::new("tx2", "Payload2");
    let mut tx3 = Transaction::new("tx3", "Payload3");
    let mut tx4 = Transaction::new("tx4", "Payload4");
    let mut tx5 = Transaction::new("tx5", "Payload5");
    let mut isolated1 = Transaction::new("isolated1", "Payload6");
    let mut isolated2 = Transaction::new("isolated2", "Payload7");
    tx1.weight = 10;
    tx2.weight = 20;
    tx3.weight = 30;
    tx4.weight = 40;
    tx5.weight = 50;
    isolated1.weight = 5;
    isolated2.weight = 5;

    tangle.add_transaction(tx1);
    tangle.add_transaction(tx2);
    tangle.add_transaction(tx3);
    tangle.add_transaction(tx4);
    tangle.add_transaction(tx5);
    tangle.add_transaction(isolated1);
    tangle.add_transaction(isolated2);

    let result = tangle.weighted_random_walk("tx1").await;

    assert!(
        result == Some("tx1".to_string())
            || result == Some("tx2".to_string())
            || result == Some("tx3".to_string())
            || result == Some("tx4".to_string())
            || result == Some("tx5".to_string()),
        "Unexpected WRW result: {:?}",
        result
    );

    let isolated_result = tangle.weighted_random_walk("isolated1").await;

    assert_eq!(isolated_result, Some("isolated1".to_string()));
}

