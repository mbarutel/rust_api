use tower_governor::governor::GovernorConfigBuilder;

pub fn rate_limit_config() -> tower_governor::governor::GovernorConfig<
    tower_governor::key_extractor::PeerIpKeyExtractor,
    governor::middleware::NoOpMiddleware,
> {
    GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(10)
        .finish()
        .expect("Failed to build rate limiter config")
}
