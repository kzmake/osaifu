fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().build_server(true).compile(
        &["../../../api/osaifu/wallet/v1/wallet.proto"],
        &["../../../api", "../../../third_party/googleapis"], // specify the root location to search proto dependencies
    )?;
    Ok(())
}
