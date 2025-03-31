# EcoWeave Library Documentation

EcoWeave is a Rust library designed to facilitate the creation, management, and validation of a distributed Tangle structure for IoT environmental data. This documentation provides an overview of its features, usage examples, and integration instructions.

---

## Table of Contents
1. [Overview](#overview)
2. [Features](#features)
    - [Create a Transaction](#1-create-a-transaction)
    - [Validate a Transaction](#2-validate-a-transaction)
    - [Sign a Transaction](#3-sign-a-transaction)
    - [Validate a Transaction's Signature](#4-validate-a-transactions-signature)
    - [Add a Transaction to the Tangle](#5-add-a-transaction-to-the-tangle)
    - [Propagate a Transaction](#6-propagate-a-transaction)
    - [Perform Weighted Random Walk (WRW)](#7-perform-weighted-random-walk-wrw)
3. [How to Use the Library](#how-to-use-the-library)
4. [Transaction Lifecycle](#transaction-lifecycle)

---

## Overview

EcoWeave provides a robust framework for managing distributed Tangle structures, enabling secure and efficient data exchange in IoT ecosystems. The library includes tools for transaction creation, validation, signing, propagation, and consensus.

---

## Features

### 1. **Create a Transaction**
Create a new transaction with a unique ID, payload, and timestamp.

**Example:**
```rust
use eco_weave::Transaction;

let tx = Transaction::new("tx1", "{\"temperature\":25.6}");
println!("{:?}", tx);
```

---

### 2. **Validate a Transaction**
Ensure the transaction meets the required format and constraints.

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
Sign a transaction using a private key to ensure authenticity.

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
Verify the authenticity of a transaction using a public key.

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
Add a validated transaction to the Tangle, ensuring no duplicates.

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
Propagate a transaction to neighboring nodes in the Tangle.

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
Select a transaction from the Tangle based on assigned weights.

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

## Transaction Lifecycle

Below is a high-level overview of the transaction lifecycle in EcoWeave:

1. **Transaction Creation**: A transaction is created and signed.
2. **Local Validation**: The transaction is validated locally.
3. **Pending Status**: The transaction is added to the local Tangle with a "pending" status.
4. **Asynchronous Propagation**: The transaction is propagated to neighboring nodes.
5. **Neighbor Validation**: Neighboring nodes validate the transaction.
6. **Consensus**: A local consensus is reached, and the transaction is confirmed.

```mermaid
graph TD
    A[Transaction Created and Signed] --> B[Local Validation]
    B --> C[Added to Local Tangle (Pending)]
    C --> D[Asynchronous Propagation]
    D --> E[Neighbor Validation]
    E --> F[Consensus Reached]
    F --> G[Transaction Confirmed]
```

---

EcoWeave simplifies the management of distributed Tangle structures, making it an ideal choice for IoT applications requiring secure and efficient data exchange.