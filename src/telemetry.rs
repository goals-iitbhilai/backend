use tracing_subscriber::EnvFilter;

/// Initialize the global tracing subscriber.
///
/// Log level is controlled by the `RUST_LOG` environment variable and
/// defaults to `info`.
pub fn init() {
    let filter = EnvFilter::builder()
        .with_default_directive(tracing::level_filters::LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt().with_env_filter(filter).init();
}
