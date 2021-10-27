use crate::postgres::models::*;
use crate::postgres::schema::wallets;
use crate::postgres::DbPool;
use anyhow::{Error, Result};
use diesel::prelude::*;
use domain::entity::*;
use domain::repository::*;
use domain::vo::*;

#[derive(Clone)]
pub struct WalletRepository {
    connections: DbPool,
}

impl WalletRepository {
    pub fn new(connections: DbPool) -> Self {
        Self { connections }
    }

    fn create_with_conn(&self, conn: &PgConnection, aggregate: Wallet) -> Result<(), Error> {
        diesel::insert_into(wallets::table)
            .values(&NewWalletModel {
                id: aggregate.id().clone().to_string(),
                owner: "alice".to_string(),
                balance: aggregate.balance().clone().to_string(),
            })
            .execute(conn)?;

        Ok(())
    }

    fn get_with_conn(&self, conn: &PgConnection, id: Id<Wallet>) -> Result<Wallet, Error> {
        let wallet = wallets::table
            .select((wallets::id, wallets::owner, wallets::balance))
            .filter(wallets::id.eq(id.to_string()))
            .first::<WalletModel>(conn)?;

        Ok(WalletBuilder::default()
            .id(wallet.id.parse::<Id<Wallet>>().unwrap())
            .balance(wallet.balance.parse::<Money<JPY>>().unwrap())
            .build()?)
    }

    fn delete_with_conn(&self, conn: &PgConnection, aggregate: Wallet) -> Result<(), Error> {
        diesel::delete(wallets::table.filter(wallets::id.eq(aggregate.id().to_string())))
            .execute(conn)?;

        Ok(())
    }
}

impl CreateRepository<Wallet> for WalletRepository {
    fn create(&self, aggregate: Wallet) -> Result<(), Error> {
        let conn = self.connections.pool().get()?;

        self.create_with_conn(&conn, aggregate)
    }
}

impl GetRepository<Wallet> for WalletRepository {
    fn get(&self, id: Id<Wallet>) -> Result<Wallet, Error> {
        let conn = self.connections.pool().get()?;

        self.get_with_conn(&conn, id)
    }
}

impl DeleteRepository<Wallet> for WalletRepository {
    fn delete(&self, aggregate: Wallet) -> Result<(), Error> {
        let conn = self.connections.pool().get()?;

        self.delete_with_conn(&conn, aggregate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;
    use ulid::Ulid;

    static INIT: Once = Once::new();

    pub fn init(connections: DbPool) {
        INIT.call_once(|| {
            connections.init().unwrap();
        });
    }

    #[test]
    fn test_wallet_repository_create() {
        let database_url = "postgres://postgres:postgres@localhost/wallet_test";
        let connections = DbPool::new(database_url);

        init(connections.clone());

        let conn = connections.pool().get().unwrap();
        conn.test_transaction::<_, Error, _>(|| {
            let entity = WalletBuilder::default()
                .id(Ulid::new().to_string().parse::<Id<Wallet>>().unwrap())
                .balance("2000".parse::<Money<JPY>>().unwrap())
                .build()
                .unwrap();

            let sut = WalletRepository::new(connections);

            assert!(sut.create_with_conn(&conn, entity).is_ok());

            Ok(())
        });
    }

    #[test]
    fn test_wallet_repository_get() {
        let database_url = "postgres://postgres:postgres@localhost/wallet_test";
        let connections = DbPool::new(database_url);

        init(connections.clone());

        let conn = connections.pool().get().unwrap();
        conn.test_transaction::<_, Error, _>(|| {
            let entity = WalletBuilder::default()
                .id(Ulid::new().to_string().parse::<Id<Wallet>>().unwrap())
                .balance("2000".parse::<Money<JPY>>().unwrap())
                .build()
                .unwrap();

            let sut = WalletRepository::new(connections);
            sut.create_with_conn(&conn, entity.clone()).unwrap();

            assert!(sut.get_with_conn(&conn, entity.id().clone()).is_ok());

            Ok(())
        });
    }

    #[test]
    fn test_wallet_repository_delete() {
        let database_url = "postgres://postgres:postgres@localhost/wallet_test";
        let connections = DbPool::new(database_url);

        init(connections.clone());

        let conn = connections.pool().get().unwrap();
        conn.test_transaction::<_, Error, _>(|| {
            let entity = WalletBuilder::default()
                .id(Ulid::new().to_string().parse::<Id<Wallet>>().unwrap())
                .balance("2000".parse::<Money<JPY>>().unwrap())
                .build()
                .unwrap();

            let sut = WalletRepository::new(connections);
            sut.create_with_conn(&conn, entity.clone()).unwrap();

            assert!(sut.delete_with_conn(&conn, entity).is_ok());

            Ok(())
        });
    }
}
