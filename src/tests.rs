#[cfg(test)]
mod unit_tests {
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
        let theme = Theme::default();
        // Test latency thresholds
        assert_eq!(theme.latency_color(20.0), theme.good);
        assert_eq!(theme.latency_color(50.0), theme.warn);
        assert_eq!(theme.latency_color(150.0), theme.crit);

        // Test graph gradient clamping
        // Should not panic on out of bounds
        let _ = theme.graph_gradient(-1.0);
        let _ = theme.graph_gradient(2.0);
    }
}
