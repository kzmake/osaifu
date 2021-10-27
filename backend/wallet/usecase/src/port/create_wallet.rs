use crate::port::{InputData, OutputData};
use derive_new::new;
use domain::entity::Wallet;

#[derive(new, Clone, Debug, PartialEq)]
pub struct CreateWalletInputData {
    pub owner: String,
}

impl InputData for CreateWalletInputData {}

#[derive(new, Clone, Debug, PartialEq)]
pub struct CreateWalletOutputData {
    pub wallet: Wallet,
}

impl OutputData for CreateWalletOutputData {}
