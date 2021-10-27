use crate::postgres::schema::wallets;

#[derive(Queryable, Debug)]
pub struct WalletModel {
    pub id: String,
    pub owner: String,
    pub balance: String,
}

#[derive(Insertable)]
#[table_name = "wallets"]
pub struct NewWalletModel {
    pub id: String,
    pub owner: String,
    pub balance: String,
}
