# eco_weave


Téléphone A (création transaction)
    ↓
Transaction créée et signée
    ↓
Validation locale
    ↓
Ajout au Tangle local (statut : pending)
    ↓
Propagation asynchrone aux voisins
    ↓
Validation chez les voisins
    ↓
Ajout au Tangle des voisins (statut : pending)
    ↓
Consensus local
    ↓
Propagation de la confirmation
    ↓
Transaction confirmée

# EcoWeave Library Features

## Overview
EcoWeave provides a set of tools to create, manage, and validate a distributed Tangle structure for IoT environmental data. Below are the features implemented so far, with examples for usage in other projects.

---

## Features

### 1. **Create a Transaction**
Allows creating a new transaction with a unique ID, payload, and timestamp.

**Example:**
```rust
use eco_weave::Transaction;

let tx = Transaction::new("tx1", "{\"temperature\":25.6}");
println!("{:?}", tx);
```

---

### 2. **Validate a Transaction**
Validates the format of a transaction, including:
- Non-empty ID and payload
- Valid ID format (alphanumeric with dashes)
- Payload size within allowed limits
- Timestamp not in the future

**Example:**
```rust
use eco_weave::Transaction;

let tx = Transaction::new("tx1", "{\"temperature\":25.6}");
match tx.validate() {
    Ok(_) => println!("Transaction is valid"),
    Err(err) => println!("Validation error: {}", err),
}
```

---

### 3. **Sign a Transaction**
Signs a transaction using a private key (`SigningKey`) to ensure authenticity.

**Example:**
```rust
use eco_weave::Transaction;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

let mut rng = OsRng;
let signing_key = SigningKey::generate(&mut rng);

let mut tx = Transaction::new("tx1", "{\"temperature\":25.6}");
tx.sign(&signing_key);

println!("Signed transaction: {:?}", tx);
```

---

### 4. **Validate a Transaction's Signature**
Verifies the signature of a transaction using a public key (`VerifyingKey`).

**Example:**
```rust
use eco_weave::Transaction;
use ed25519_dalek::VerifyingKey;
use rand::rngs::OsRng;

let mut rng = OsRng;
let signing_key = SigningKey::generate(&mut rng);
let verifying_key = signing_key.verifying_key();

let mut tx = Transaction::new("tx1", "{\"temperature\":25.6}");
tx.sign(&signing_key);

match tx.validate_signature(&verifying_key) {
    Ok(_) => println!("Signature is valid"),
    Err(err) => println!("Invalid signature: {}", err),
}
```

---

### 5. **Add a Transaction to the Tangle**
Adds a validated transaction to the Tangle, ensuring no duplicates.

**Example:**
```rust
use eco_weave::{Tangle, Transaction};

let mut tangle = Tangle::new();
let tx = Transaction::new("tx1", "{\"temperature\":25.6}");

if tangle.add_transaction(tx) {
    println!("Transaction added to the Tangle.");
} else {
    println!("Failed to add transaction.");
}
```

---

### 6. **Propagate a Transaction**
Propagates a transaction to neighboring nodes in the Tangle.

**Example:**
```rust
use eco_weave::{Tangle, Transaction};

let mut tangle = Tangle::new();
tangle.add_node("node1");
tangle.add_node("node2");
tangle.connect_nodes("node1", "node2");

let tx = Transaction::new("tx1", "{\"temperature\":25.6}");
tangle.add_transaction(tx.clone());

let propagated_count = tangle.propagate_transaction(tx, "node1");
println!("Transaction propagated to {} nodes.", propagated_count);
```

---

### 7. **Perform Weighted Random Walk (WRW)**
Selects a transaction from the Tangle based on weights assigned to transactions.

**Example:**
```rust
use eco_weave::{Tangle, Transaction};

let mut tangle = Tangle::new();
tangle.add_node("node1");
tangle.add_node("node2");
tangle.connect_nodes("node1", "node2");

let mut tx1 = Transaction::new("tx1", "{\"temperature\":25.6}");
tx1.weight = 10; // Assign weight
tangle.add_transaction(tx1);

let mut tx2 = Transaction::new("tx2", "{\"humidity\":60.5}");
tx2.weight = 20; // Assign weight
tangle.add_transaction(tx2);

if let Some(selected_tx) = tangle.weighted_random_walk("node1") {
    println!("Selected transaction: {}", selected_tx);
} else {
    println!("No transaction selected.");
}
```

---

## How to Use the Library

### 1. Add EcoWeave to Your Project
Add the library to your `Cargo.toml`:
```toml
[dependencies]
eco_weave = { path = "../eco_weave" }
```

### 2. Import the Library
Use the provided modules and structs in your Rust code:
```rust
use eco_weave::{Tangle, Transaction};
```

---