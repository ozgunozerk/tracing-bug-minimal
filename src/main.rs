use tracing::level_filters::LevelFilter;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::prelude::*;
use tracing_subscriber::Layer;
use tracing_subscriber::{fmt, fmt::format::FmtSpan, EnvFilter};

fn main() {
    let log_dir = "tracing-test/";

    let filter = || {
        EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy()
    };

    let mut file_appender = tracing_appender::rolling::minutely(log_dir, format!("{}.log", "heyo"));
    file_appender.keep_last_n_logs(2);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // start logger, after we acquire the bundle identifier
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_ansi(false)
                .with_span_events(FmtSpan::CLOSE)
                .with_filter(filter()),
        )
        .with(
            BunyanFormattingLayer::new("some-dependency".to_owned(), non_blocking)
                .and_then(JsonStorageLayer)
                .with_filter(filter()),
        )
        .init();
}
