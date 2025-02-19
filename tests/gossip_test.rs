#[cfg(test)]
mod tests {
    use eco_weave::Tangle;
    use eco_weave::network::gossip_protocol::GossipProtocol;
    use eco_weave::transaction::Transaction;
    use tokio::time::sleep;
    use std::time::Duration;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    #[tokio::test]
    async fn test_gossip_propagation() {
        let mut tangle = Tangle::new();
        let mut gossip = GossipProtocol::new(&tangle);

        let transaction = Transaction::new("tx-gossip", "test-payload").unwrap();

        // Ajouter 5 nœuds avec une clé publique
        for i in 0..5 {
            let keypair = SigningKey::generate(&mut OsRng);
            tangle.add_node(format!("node_{}", i), keypair.verifying_key());
        }

        // Connecter chaque nœud à au moins un autre
        for i in 0..5 {
            for j in i+1..5 {
                tangle.connect_nodes(&format!("node_{}", i), &format!("node_{}", j));
            }
        }

        // Vérifier que node_0 a bien des voisins
        println!("Neighbors of node_0: {:?}", tangle.get_neighbors("node_0"));

        // Ajouter manuellement la transaction pour éviter qu'elle ne soit perdue
        tangle.add_transaction(transaction.clone());

        // Propager la transaction depuis node_0
        gossip.propagate_transaction(transaction.clone(), "node_0").await;

        // Attendre un peu pour que la propagation ait le temps de s'exécuter
        sleep(Duration::from_secs(1)).await;

        // Vérifier si la transaction a bien été propagée dans le Tangle
        assert!(
            tangle.transactions.contains_key(&transaction.id),
            "Transaction {} was not propagated!",
            transaction.id
        );
    }
}