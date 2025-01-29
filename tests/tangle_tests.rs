use ed25519_dalek::{SigningKey, VerifyingKey};
use eco_weave::{Node, Tangle, Transaction};
use rand::rngs::OsRng;

#[test]
fn test_add_node() {
    let mut tangle = Tangle::new();

    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();

    // Ajoute un nœud
    assert!(tangle.add_node("node1", verifying_key));

    // Vérifie que le nœud est présent dans le Tangle
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

    // Connecte les nœuds
    assert!(tangle.connect_nodes("node1", "node2"));

    // Vérifie les connexions
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

    // Crée une transaction valide
    let mut transaction = Transaction::new("node1", "payload").unwrap();
    transaction.sign(&signing_key); // Signature de la transaction

    // Ajoute la transaction au Tangle
    assert!(tangle.add_transaction(transaction.clone()));

    // Vérifie que la transaction est bien ajoutée
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

    // Connect the nodes
    tangle.connect_nodes("node1", "node2");

    // Create and sign a transaction
    let mut transaction = Transaction::new("node1", "test_payload").unwrap();
    transaction.sign(&signing_key1); // Sign the transaction

    // Propagate the transaction
    let propagated_count = tangle.propagate_transaction(transaction.clone(), "node1").await;

    // Verify that the transaction is propagated
    assert_eq!(propagated_count, 2);
    assert!(tangle.transactions.contains_key("node1"));
}