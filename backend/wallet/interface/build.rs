use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let descriptor = PathBuf::from(env::var("OUT_DIR").unwrap()).join("osaifu.wallet.v1.bin");
    tonic_build::configure()
        .file_descriptor_set_path(&descriptor)
        .build_server(true)
        .build_client(true)
        .format(true)
        .compile(
            &["../../../api/osaifu/wallet/v1/wallet.proto"],
            &["../../../api", "../../../third_party/googleapis"], // specify the root location to search proto dependencies
        )?;
    Ok(())
}
