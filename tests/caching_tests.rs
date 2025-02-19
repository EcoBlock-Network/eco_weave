

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::thread::sleep;
    use eco_weave::resilience::caching::TransactionCache;


    #[test]
    fn test_transaction_cache() {
        let mut cache = TransactionCache::new(Duration::from_secs(2));

        cache.add_transaction("tx_1".to_string(), "payload_1".to_string());
        cache.add_transaction("tx_2".to_string(), "payload_2".to_string());

        assert_eq!(cache.cache.len(), 2, "Cache should contain 2 transactions");

        sleep(Duration::from_secs(3));
        cache.cleanup_expired();

        assert_eq!(cache.cache.len(), 0, "Cache should be empty after expiration");
    }
}