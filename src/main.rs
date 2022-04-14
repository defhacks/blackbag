use tracing::trace;
use color_eyre::eyre;

fn main() -> eyre::Result<()> {
    install_tracing();
    trace!("tracing installed.");
    color_eyre::install()?;
    trace!("color_eyre installed.");

    blackbag::start()
}

fn install_tracing() {
use tracing_subscriber::prelude::*;
use tracing_subscriber::fmt::{format,layer,time};
use tracing_subscriber::EnvFilter;
use tracing_error::ErrorLayer;

    let fmt_layer = layer()
        .with_target(false)
        .with_span_events(format::FmtSpan::ACTIVE)
        .with_writer(std::io::stderr)
        .with_timer(time::LocalTime::rfc_3339());
    let filter_layer = EnvFilter::try_from_env("BKB_LOG")
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}
