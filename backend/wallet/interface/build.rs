fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../../../api/osaifu/wallet/v1/wallet.proto")?;

    Ok(())
}
