use std::time::Duration;

// link the crates whose components register through `linkme`, even though
// nothing here names them yet
use uju_demo_model as _;
use uju_demo_sim as _;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    uju::init();

    let builder = uju::runtime::node::Builder::new(0)
        .with_shard(uju::runtime::shard::Builder::new().tick_interval(Duration::from_secs(1)));
    builder.run()?;

    Ok(())
}
