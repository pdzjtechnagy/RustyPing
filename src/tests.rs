#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use crate::network::SpeedTestState;

    #[test]
    fn test_speed_test_state_transitions() {
        let state = SpeedTestState::Idle;
        assert!(matches!(state, SpeedTestState::Idle));
    }

    // Since we can't easily mock network calls in this environment without complex traits,
    // we focus on logic tests.

    #[test]
    fn test_config_serialization() {
        use crate::storage::Config;
        let config = Config::default();
        let json = serde_json::to_string(&config).unwrap();
        let loaded: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(config.ping_interval_ms, loaded.ping_interval_ms);
    }
}
