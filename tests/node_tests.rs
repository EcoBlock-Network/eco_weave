use eco_weave::Node;
use ed25519_dalek::SigningKey;

#[test]
fn test_node_creation() {
    // Generate a private key (SigningKey) and obtain the public key (VerifyingKey)
    let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();

    // Create a node with an ID and the public key
    let node = Node::new("node1", verifying_key);

    // Verify that the ID and the list of neighbors are correctly initialized
    assert_eq!(node.id, "node1");
    assert!(node.neighbors.is_empty());
}

#[test]
fn test_add_neighbor() {
    // Generate a private key (SigningKey) and obtain the public key (VerifyingKey)
    let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();

    let mut node = Node::new("node1", verifying_key);

    // Add a neighbor to the node
    node.add_neighbor("node2");

    // Verify that the neighbor has been added
    assert_eq!(node.neighbors.len(), 1);
    assert!(node.is_neighbor("node2"));
}

#[test]
fn test_no_duplicate_neighbors() {
    // Generate a private key (SigningKey) and obtain the public key (VerifyingKey)
    let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();

    let mut node = Node::new("node1", verifying_key);

    // Add a neighbor twice
    node.add_neighbor("node2");
    node.add_neighbor("node2");

    // Verify that there are no duplicates
    assert_eq!(node.neighbors.len(), 1);
}

#[test]
fn test_is_neighbor() {
    // Generate a private key (SigningKey) and obtain the public key (VerifyingKey)
    let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();

    let mut node = Node::new("node1", verifying_key);

    // Add a neighbor
    node.add_neighbor("node2");

    // Verify that the neighbor is recognized as such
    assert!(node.is_neighbor("node2"));

    // Verify that a non-added neighbor is not recognized
    assert!(!node.is_neighbor("node3"));
}

#[test]
fn test_node_verifying_key() {
    // Generate a private key (SigningKey) and obtain the public key (VerifyingKey)
    let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();

    // Create a node with an ID and a public key
    let node = Node::new("node1", verifying_key);

    // Verify that the public key is correct
    assert_eq!(node.verifying_key, verifying_key);
}

#[test]
fn test_add_multiple_neighbors() {
    let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();

    let mut node = Node::new("node1", verifying_key);

    // Add multiple neighbors
    node.add_neighbor("node2");
    node.add_neighbor("node3");

    // Verify that the neighbors have been added
    assert_eq!(node.neighbors.len(), 2);
    assert!(node.is_neighbor("node2"));
    assert!(node.is_neighbor("node3"));
}

#[test]
fn test_node_invalid_id() {
    let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();

    // Create a node with an empty ID
    let node = Node::new("", verifying_key);

    // Verify that the ID is stored as is (according to the logic defined in Node::new)
    assert_eq!(node.id, "");
}

#[test]
fn test_node_prevent_self_neighbor() {
    let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
    let verifying_key = signing_key.verifying_key();

    let mut node = Node::new("node1", verifying_key);

    // Add the node itself as a neighbor
    node.add_neighbor("node1");

    // Verify that the node is not its own neighbor
    assert_eq!(node.neighbors.len(), 0);
    assert!(!node.is_neighbor("node1"));
}
