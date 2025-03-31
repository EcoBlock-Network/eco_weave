use eco_weave::{Tangle, Transaction};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

#[test]
fn test_add_node() {
    let mut tangle = Tangle::new();

    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();

    assert!(tangle.add_node("node1", verifying_key));

    assert!(tangle.nodes.contains_key("node1"));
}

#[test]
fn test_connect_nodes() {
    let mut tangle = Tangle::new();
    let signing_key1 = SigningKey::generate(&mut OsRng);
    let verifying_key1 = signing_key1.verifying_key();
    let signing_key2 = SigningKey::generate(&mut OsRng);
    let verifying_key2 = signing_key2.verifying_key();

    tangle.add_node("node1", verifying_key1);
    tangle.add_node("node2", verifying_key2);

    assert!(tangle.connect_nodes("node1", "node2"));

    let node1 = tangle.nodes.get("node1").unwrap();
    let node2 = tangle.nodes.get("node2").unwrap();

    assert!(node1.is_neighbor("node2"));
    assert!(node2.is_neighbor("node1"));
}

#[test]
fn test_add_transaction() {
    let mut tangle = Tangle::new();
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();

    tangle.add_node("node1", verifying_key);

    let mut transaction = Transaction::new("node1", "payload").unwrap();
    transaction.sign(&signing_key);

    assert!(tangle.add_transaction(transaction.clone()));
    assert!(tangle.transactions.contains_key("node1"));
}

#[tokio::test]
async fn test_propagate_transaction() {
    let mut tangle = Tangle::new();

    let signing_key1 = SigningKey::generate(&mut OsRng);
    let verifying_key1 = signing_key1.verifying_key();
    let signing_key2 = SigningKey::generate(&mut OsRng);
    let verifying_key2 = signing_key2.verifying_key();

    tangle.add_node("node1", verifying_key1);
    tangle.add_node("node2", verifying_key2);
    tangle.connect_nodes("node1", "node2");

    let mut transaction = Transaction::new("node1", "test_payload").unwrap();
    transaction.sign(&signing_key1);
    let propagated_count = tangle
        .propagate_transaction(transaction.clone(), "node1")
        .await;

    assert_eq!(propagated_count, 2);
    assert!(tangle.transactions.contains_key("node1"));
}

#[test]
fn test_get_snapshot() {
    let mut rng = OsRng;
    let signing_key = SigningKey::generate(&mut rng);
    let verifying_key = signing_key.verifying_key();

    let mut tangle = Tangle::new();

    tangle.add_node("txn-1", verifying_key);
    tangle.add_node("txn-2", verifying_key);
    tangle.add_node("txn-3", verifying_key);

    let mut tx1 = Transaction::new("txn-1", "Hello 1").unwrap();
    let mut tx2 = Transaction::new("txn-2", "Hello 2").unwrap();
    let mut tx3 = Transaction::new("txn-3", "Hello 3").unwrap();

    tx1.sign(&signing_key);
    tx2.sign(&signing_key);
    tx3.sign(&signing_key);

    tangle.add_transaction(tx1);
    tangle.add_transaction(tx2);
    tangle.add_transaction(tx3);

    tangle.connect_nodes("txn-1", "txn-2");
    tangle.connect_nodes("txn-2", "txn-3");

    let snapshot = tangle.get_snapshot();

    assert_eq!(snapshot.len(), 3);

    let txn_1_neighbors = snapshot
        .iter()
        .find(|(id, _)| id == "txn-1")
        .unwrap()
        .1
        .clone();
    let txn_2_neighbors = snapshot
        .iter()
        .find(|(id, _)| id == "txn-2")
        .unwrap()
        .1
        .clone();
    let txn_3_neighbors = snapshot
        .iter()
        .find(|(id, _)| id == "txn-3")
        .unwrap()
        .1
        .clone();

    assert!(txn_1_neighbors.contains(&"txn-2".to_string()));
    assert!(txn_2_neighbors.contains(&"txn-1".to_string()));
    assert!(txn_2_neighbors.contains(&"txn-3".to_string()));
    assert!(txn_3_neighbors.contains(&"txn-2".to_string()));
}
