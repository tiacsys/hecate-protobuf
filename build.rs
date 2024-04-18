use anyhow::Result;
use prost_build_config::{BuildConfig, Builder};

fn main() -> Result<()> {
    let config: BuildConfig = serde_yaml::from_str(include_str!("./prost-build-config.yml"))?;
    Builder::from(config).build_protos();
    Ok(())
}
