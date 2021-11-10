use crate::port::{InputData, OutputData};
use derive_new::new;

#[derive(new, Clone, Debug, PartialEq)]
pub struct DeleteWalletInputData {
    pub id: String,
}

impl InputData for DeleteWalletInputData {}

#[derive(new, Clone, Debug, PartialEq)]
pub struct DeleteWalletOutputData {}

impl OutputData for DeleteWalletOutputData {}
