use anyhow::Result;
use std::{
    env,
    fs::create_dir,
    io::{Error, ErrorKind},
    path::Path,
};

fn main() -> Result<()> {
    let out_dir = format!(
        "{}/protos",
        env::var("OUT_DIR").map_err(|err| Error::other(err.to_string()))?
    );
    create_dir(Path::new(&out_dir)).or_else(|err| match err.kind() {
        ErrorKind::AlreadyExists => Ok(()),
        _ => Err(err),
    })?;

    tonic_prost_build::configure().compile_protos(&["../../ens/ens.proto"], &["../../ens"])?;

    Ok(())
}
