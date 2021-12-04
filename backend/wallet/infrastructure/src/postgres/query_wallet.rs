use crate::postgres::models::*;
use crate::postgres::schema::wallets;
use crate::postgres::DbPool;
use anyhow::{Error, Result};
use diesel::prelude::*;
use query::repository::*;
use query::view::*;

#[derive(Clone)]
pub struct QueryWalletRepository {
    connections: DbPool,
}

impl QueryWalletRepository {
    pub fn new(connections: DbPool) -> Self {
        Self { connections }
    }

    fn list_with_conn(&self, conn: &PgConnection) -> Result<Vec<Wallet>, Error> {
        let wallets = wallets::table
            .select((wallets::id, wallets::owner, wallets::balance))
            .get_results::<WalletModel>(conn)?;

        Ok(wallets
            .iter()
            .map(|w| Wallet {
                id: w.id.clone(),
                owner: w.owner.clone(),
                balance: w.balance.clone(),
            })
            .collect())
    }

    fn get_with_conn(&self, conn: &PgConnection, id: String) -> Result<Wallet, Error> {
        let wallet = wallets::table
            .select((wallets::id, wallets::owner, wallets::balance))
            .filter(wallets::id.eq(id))
            .first::<WalletModel>(conn)?;

        Ok(Wallet {
            id: wallet.id,
            owner: wallet.owner,
            balance: wallet.balance,
        })
    }
}

impl ListRepository<Wallet> for QueryWalletRepository {
    fn list(&self) -> Result<Vec<Wallet>, Error> {
        let conn = self.connections.pool().get()?;

        self.list_with_conn(&conn)
    }
}

impl GetRepository<Wallet> for QueryWalletRepository {
    fn get(&self, id: String) -> Result<Wallet, Error> {
        let conn = self.connections.pool().get()?;

        self.get_with_conn(&conn, id)
    }
}
