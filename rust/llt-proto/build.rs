use anyhow::Result;

const PROTO_PATH: &str = "../../ens/ens.proto";

fn main() -> Result<()> {
    tonic_prost_build::configure().compile_protos(&[PROTO_PATH], &["../../ens"])?;

    // This makes it possible to find the 'ens.proto' in the target directory
    let out = std::env::var("OUT_DIR")?;
    std::fs::copy(PROTO_PATH, format!("{out}/ens.proto"))?;

    Ok(())
}
