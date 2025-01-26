use eco_weave::{Transaction, Tangle};

#[test]
fn test_add_transaction() {
    let mut tangle = Tangle::new();
    let tx = Transaction::new("tx1", r#"{"temperature": 25}"#).unwrap();
    assert!(tangle.add_transaction(tx.clone()));
    assert_eq!(tangle.transactions.len(), 1);
    assert_eq!(tangle.transactions["tx1"], tx);
}

#[test]
fn test_duplicate_transaction() {
    let mut tangle = Tangle::new();
    let tx = Transaction::new("tx1", r#"{"humidity": 50}"#).unwrap();
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

    let tx = Transaction::new("tx1", r#"{"pressure": 1013}"#).unwrap();

    let propagated_count = tangle.propagate_transaction(tx.clone(), "node1").await;
    assert_eq!(propagated_count, 3);

    assert!(tangle.transactions.contains_key("tx1"));
    assert_eq!(tangle.transactions.len(), 1);
}

#[test]
fn test_add_valid_transaction() {
    let mut tangle = Tangle::default();
    let tx = Transaction::new("tx1", r#"{"wind_speed": 10}"#).unwrap();
    assert!(tangle.add_transaction(tx));
}

#[test]
fn test_add_invalid_transaction_empty_id() {
    let tx = Transaction::new("", r#"{"uv_index": 5}"#);
    assert!(tx.is_err());
}

#[test]
fn test_add_invalid_transaction_empty_payload() {
    let tx = Transaction::new("tx2", "");
    assert!(tx.is_err());
}

#[tokio::test]
async fn test_weighted_random_walk() {
    let mut tangle = Tangle::new();

    tangle.add_node("tx1");
    tangle.add_node("tx2");

    tangle.connect_nodes("tx1", "tx2");

    let mut tx1 = Transaction::new("tx1", r#"{"temperature": 20}"#).unwrap();
    let mut tx2 = Transaction::new("tx2", r#"{"humidity": 40}"#).unwrap();
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
