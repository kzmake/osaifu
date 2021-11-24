use crate::port::{ListWalletsInputData, ListWalletsOutputData, Port};
use crate::repository::ListRepository;
use crate::view::Wallet;
use anyhow::{Error, Result};
use derive_new::new;

#[derive(new)]
pub struct ListWalletsInteractor<Query>
where
    Query: ListRepository<Wallet>,
{
    wallet_repository: Query,
}

impl<Query> Port<ListWalletsInputData, ListWalletsOutputData> for ListWalletsInteractor<Query>
where
    Query: ListRepository<Wallet>,
{
    fn handle(&self, _input: ListWalletsInputData) -> Result<ListWalletsOutputData, Error> {
        let wallets = self.wallet_repository.list()?;

        Ok(ListWalletsOutputData::new(wallets))
    }
}
