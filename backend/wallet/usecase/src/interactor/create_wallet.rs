use crate::port::{CreateWalletInputData, CreateWalletOutputData, Port};
use anyhow::{Error, Result};
use derive_new::new;
use domain::entity::{Wallet, WalletBuilder};
use domain::repository::IdRepository;
use domain::repository::{CreateRepository, GetRepository};
use domain::vo::{Money, JPY};

#[derive(new)]
pub struct CreateWalletInteractor<I, S>
where
    I: IdRepository,
    S: CreateRepository<Wallet> + GetRepository<Wallet>,
{
    id_repository: I,
    wallet_repository: S,
}

impl<I, S> Port<CreateWalletInputData, CreateWalletOutputData> for CreateWalletInteractor<I, S>
where
    I: IdRepository,
    S: CreateRepository<Wallet> + GetRepository<Wallet>,
{
    fn handle(&self, _input: CreateWalletInputData) -> Result<CreateWalletOutputData, Error> {
        let id = self.id_repository.generate::<Wallet>()?;

        let wallet = WalletBuilder::default()
            .id(id)
            .balance("0".parse::<Money<JPY>>()?)
            .build()
            .unwrap();

        self.wallet_repository.create(wallet.clone())?;
        match self.wallet_repository.get(wallet.id().clone()) {
            Ok(aggregate_root) => Ok(CreateWalletOutputData::new(aggregate_root)),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::bail;
    use domain::repository::DeleteRepository;
    use domain::vo::Id;
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::sync::Mutex;

    #[derive(new)]
    struct MockIdRepository {}
    impl IdRepository for MockIdRepository {
        fn generate<T>(&self) -> Result<Id<T>, Error> {
            Ok("01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id<T>>().unwrap())
        }
    }

    struct MockWalletRepository {
        store: Arc<Mutex<HashMap<Id<Wallet>, Wallet>>>,
    }
    impl MockWalletRepository {
        fn new() -> Self {
            let m = HashMap::new();
            Self {
                store: Arc::new(Mutex::new(m)),
            }
        }
    }

    impl CreateRepository<Wallet> for MockWalletRepository {
        fn create(&self, entity: Wallet) -> Result<(), Error> {
            let mut m = self.store.lock().unwrap();
            m.insert(entity.id().clone(), entity.clone());
            Ok(())
        }
    }

    impl GetRepository<Wallet> for MockWalletRepository {
        fn get(&self, id: Id<Wallet>) -> Result<Wallet, Error> {
            let m = self.store.lock().unwrap();
            match m.get(&id.clone()) {
                Some(aggregate_root) => Ok(aggregate_root.clone()),
                None => bail!("not found entity"),
            }
        }
    }

    impl DeleteRepository<Wallet> for MockWalletRepository {
        fn delete(&self, entity: Wallet) -> Result<(), Error> {
            let mut m = self.store.lock().unwrap();
            match m.remove(&entity.id().clone()) {
                Some(_) => Ok(()),
                None => bail!("not found entity"),
            }
        }
    }

    #[test]
    fn test_create_wallet_handle() {
        let id_repository = MockIdRepository::new();
        let wallet_repository = MockWalletRepository::new();
        let sut = CreateWalletInteractor::new(id_repository, wallet_repository);

        assert_eq!(
            sut.handle(CreateWalletInputData::new("alice".to_string())).unwrap(),
            CreateWalletOutputData::new(
                WalletBuilder::default()
                    .id("01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id<Wallet>>().unwrap())
                    .balance("0".parse::<Money<JPY>>().unwrap())
                    .build()
                    .unwrap(),
            )
        );

        // ok
        assert!(sut.handle(CreateWalletInputData::new("alice".to_string())).is_ok());

        // err
        // assert!(sut.handle(CreateWalletInputData::new()).is_err());
    }
}
