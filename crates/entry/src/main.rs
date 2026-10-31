// link the model's components, which register through `linkme`, even though
// nothing here names them yet
use uju_demo_model as _;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    uju::init();

    Ok(())
}
