use crate::port::{GetWalletInputData, GetWalletOutputData, Port};
use anyhow::{Error, Result};
use derive_new::new;
use domain::entity::Wallet;
use domain::repository::GetRepository;
use domain::vo::Id;

#[derive(new)]
pub struct GetWalletInteractor<S>
where
    S: GetRepository<Wallet>,
{
    wallet_repository: S,
}

impl<S> Port<GetWalletInputData, GetWalletOutputData> for GetWalletInteractor<S>
where
    S: GetRepository<Wallet>,
{
    fn handle(&self, input: GetWalletInputData) -> Result<GetWalletOutputData, Error> {
        let id = input.id.parse::<Id<Wallet>>()?;

        let wallet = self.wallet_repository.get(id)?;

        Ok(GetWalletOutputData::new(wallet))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::bail;
    use domain::entity::WalletBuilder;
    use domain::repository::CreateRepository;
    use domain::repository::DeleteRepository;
    use domain::repository::GetRepository;
    use domain::vo::Id;
    use domain::vo::JPY;
    use domain::vo::Money;
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::sync::Mutex;

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
    fn test_get_wallet_handle() {
        let wallet_repository = MockWalletRepository::new();
        let wallet_a = WalletBuilder::default()
            .id("01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id<Wallet>>().unwrap())
            .balance("1000".parse::<Money<JPY>>().unwrap())
            .build()
            .unwrap();
        let wallet_b = WalletBuilder::default()
            .id("01F8MECHZX3TBDSZ7XRADM79XF".parse::<Id<Wallet>>().unwrap())
            .balance("0".parse::<Money<JPY>>().unwrap())
            .build()
            .unwrap();

        wallet_repository.create(wallet_a.clone()).unwrap();
        wallet_repository.create(wallet_b.clone()).unwrap();

        let sut = GetWalletInteractor::new(wallet_repository);

        assert_eq!(
            sut.handle(GetWalletInputData::new(wallet_a.id().to_string()))
                .unwrap(),
            GetWalletOutputData::new(wallet_a.clone())
        );

        // ok
        assert!(sut
            .handle(GetWalletInputData::new(wallet_b.id().to_string()))
            .is_ok());

        // err
        assert!(sut
            .handle(GetWalletInputData::new("NOTFOUND_ID".to_string()))
            .is_err());
    }
}
