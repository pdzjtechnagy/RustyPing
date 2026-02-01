#[cfg(test)]
mod tests {
    use crate::network::SpeedTestState;
    use crate::theme::Theme;

    #[test]
    fn test_speed_test_state_transitions() {
        let state = SpeedTestState::Preparing;
        assert!(matches!(state, SpeedTestState::Preparing));
    }

    #[test]
    fn test_config_serialization() {
        use crate::storage::Config;
        let config = Config::default();
        let json = serde_json::to_string(&config).unwrap();
        let loaded: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(config.ping_interval_ms, loaded.ping_interval_ms);
    }

    #[test]
    fn test_theme_colors() {
        // Test latency thresholds
        assert_eq!(Theme::latency_color(20.0), Theme::GOOD);
        assert_eq!(Theme::latency_color(50.0), Theme::WARN);
        assert_eq!(Theme::latency_color(150.0), Theme::CRIT);

        // Test graph gradient clamping
        // Should not panic on out of bounds
        let _ = Theme::graph_gradient(-1.0);
        let _ = Theme::graph_gradient(2.0);
    }
}
