use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

// NOTE: that we use the impl trait as the return type which means that checks happen at runtime and not compile time

// sink is the place that we want to output the logs to
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Sync + Send
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // we first try to use the env filter that is set to env (this is RUST_LOG) if it cannot use it then we use the env filter
    // that one provides to the function
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    // here we create the subscriber (Registry is a subscriber implementation) where we provide layers which is tracing terminology
    // where one implements behaviour for recording and collectings traces, multiple layers can be used together so
    // we can create the subscriber,
    // note for bunyan we have the json layer and the formatting layer which is used to create the bunyan compatible format log
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    // LogTracer is our log::Log implementaiton provided by log crate that takes log::Records and converts them into tracing::Event
    // which means that the logs from log crate is also included
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber")
}
