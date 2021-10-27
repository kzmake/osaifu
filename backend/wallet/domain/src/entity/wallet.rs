use crate::entity::Entity;
use crate::vo::Id;
use crate::vo::{Money, JPY};
use derive_builder::Builder;
use getset::{Getters, Setters};

#[derive(Default, Clone, Debug, Getters, Setters, Builder, Eq)]
#[builder(setter(into))]
pub struct Wallet {
    #[builder(pattern = "immutable")]
    #[getset(get = "pub")]
    id: Id<Wallet>,

    #[getset(get = "pub", set = "pub")]
    balance: Money<JPY>,
}

impl Entity for Wallet {
    type Id = Id<Wallet>;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}

impl PartialEq for Wallet {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet() {
        assert!(WalletBuilder::default()
            .id("01F8MECHZX3TBDSZ7XRADM79XE"
                .parse::<Id::<Wallet>>()
                .unwrap())
            .balance("1000".parse::<Money<JPY>>().unwrap())
            .build()
            .is_ok());
    }

    #[test]
    fn test_wallet_eq() {
        let before = WalletBuilder::default()
            .id("01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id<Wallet>>().unwrap())
            .balance("1000".parse::<Money<JPY>>().unwrap())
            .build()
            .unwrap();

        let mut after = before.clone();
        after.set_balance("1000".parse::<Money<JPY>>().unwrap());

        let another = WalletBuilder::default()
            .id("01F8MECHZX3TBDSZ7XRADM79XF".parse::<Id<Wallet>>().unwrap())
            .balance("1000".parse::<Money<JPY>>().unwrap())
            .build()
            .unwrap();

        assert_eq!(before, after);
        assert_ne!(before, another);
        assert_ne!(after, another);
    }
}
