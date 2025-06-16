use const_format::formatcp;
use nameof::name_of;
use tracing::{Subscriber, subscriber::set_global_default};
use tracing_bunyan_formatter::{
    BunyanFormattingLayer, JsonStorageLayer,
};
use tracing_log::LogTracer;
use tracing_subscriber::{
    EnvFilter, Registry, fmt::MakeWriter,
    layer::SubscriberExt,
};

pub fn get_subscriber(
    name: String,
    env_filter: String,
    sink: impl for<'a> MakeWriter<'a> + Send + Sync + 'static,
) -> impl Subscriber + Send + Sync {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer =
        BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(
    subscriber: impl Subscriber + Send + Sync,
) {
    LogTracer::init().expect(formatcp!(
        "Failed to set '{}'",
        name_of!(type LogTracer)
    ));
    set_global_default(subscriber).expect(formatcp!(
        "Failed to {} subscriber.",
        stringify!(tracing::subscriber::set_global_default)
    ));
}
