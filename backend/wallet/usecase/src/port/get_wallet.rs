use crate::port::{InputData, OutputData};
use derive_new::new;
use domain::entity::Wallet;

#[derive(new, Clone, Debug, PartialEq)]
pub struct GetWalletInputData {
    pub id: String,
}

impl InputData for GetWalletInputData {}

#[derive(new, Clone, Debug, PartialEq)]
pub struct GetWalletOutputData {
    pub wallet: Wallet,
}

impl OutputData for GetWalletOutputData {}
