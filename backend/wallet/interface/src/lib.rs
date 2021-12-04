pub mod osaifu_wallet_v1 {
    tonic::include_proto!("osaifu.wallet.v1");
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("osaifu.wallet.v1");
}

pub mod controller;
