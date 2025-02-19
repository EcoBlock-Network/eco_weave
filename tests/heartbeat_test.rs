use eco_weave::resilience::heartbeat::HeartbeatMonitor;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_heartbeat_detection() {
        let mut monitor = HeartbeatMonitor::new(Duration::from_secs(2));

        monitor.update_heartbeat("node_1".to_string());
        monitor.update_heartbeat("node_2".to_string());

        sleep(Duration::from_secs(1)).await;
        let inactive_nodes = monitor.check_inactive_nodes().await;
        assert!(inactive_nodes.is_empty(), "No nodes should be inactive yet");

        sleep(Duration::from_secs(3)).await;
        let inactive_nodes = monitor.check_inactive_nodes().await;
        assert_eq!(inactive_nodes.len(), 2, "Both nodes should be inactive");
    }
}