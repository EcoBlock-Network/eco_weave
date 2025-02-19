#[cfg(test)]
mod tests {
    use eco_weave::resilience::caching::TransactionCache;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_transaction_cache() {
        let mut cache = TransactionCache::new(Duration::from_secs(2));

        cache.add_transaction("tx_1".to_string(), "payload_1".to_string());
        cache.add_transaction("tx_2".to_string(), "payload_2".to_string());

        assert_eq!(cache.cache.len(), 2, "Cache should contain 2 transactions");

        sleep(Duration::from_secs(3));
        cache.cleanup_expired();

        assert_eq!(
            cache.cache.len(),
            0,
            "Cache should be empty after expiration"
        );
    }

    #[test]
    //This test is to simulate a scenario where the cache is under load and transactions are being added to it.
    fn test_transaction_cache_under_load() {
        let mut cache = TransactionCache::new(Duration::from_secs(2));

        for i in 0..10_000 {
            cache.add_transaction(format!("tx_{}", i), format!("payload_{}", i));
        }

        assert_eq!(
            cache.cache.len(),
            10_000,
            "Cache should contain 10,000 transactions"
        );

        let _ = sleep(Duration::from_secs(3));
        cache.cleanup_expired();

        assert_eq!(
            cache.cache.len(),
            0,
            "Cache should be empty after expiration"
        );
    }
}
