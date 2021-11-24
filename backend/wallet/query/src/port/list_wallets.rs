use crate::port::{InputData, OutputData};
use crate::view::Wallet;
use derive_new::new;

#[derive(new, Clone, Debug, PartialEq)]
pub struct ListWalletsInputData {
    pub owner: String,
}

impl InputData for ListWalletsInputData {}

#[derive(new, Clone, Debug)]
pub struct ListWalletsOutputData {
    pub wallets: Vec<Wallet>,
}

impl OutputData for ListWalletsOutputData {}
