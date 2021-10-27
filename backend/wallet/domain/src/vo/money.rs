use crate::vo::ValueObject;
use anyhow::Error;
use rust_decimal::Decimal;
use std::marker::PhantomData;
use std::str::FromStr;

#[allow(dead_code)]
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct JPY {}

#[allow(dead_code)]
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct USD {}

#[derive(Default, Clone, Debug, Eq)]
pub struct Money<E>(Decimal, PhantomData<E>);

impl<E> ValueObject for Money<E> {}

impl<E> FromStr for Money<E> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Decimal::from_str(s)?, PhantomData))
    }
}

impl<T> ToString for Money<T> {
    fn to_string(&self) -> String {
        self.0.clone().to_string()
    }
}

impl<T> PartialEq for Money<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_from_str() {
        assert!("1000".parse::<Money::<JPY>>().is_ok());
        assert!("1000".parse::<Money::<USD>>().is_ok());
    }

    #[test]
    fn test_money_to_string() {
        assert_eq!(
            "1000".parse::<Money::<JPY>>().unwrap().to_string(),
            "1000".to_string()
        );
    }

    #[test]
    fn test_money_eq() {
        let jpy = "1000".parse::<Money<JPY>>().unwrap();
        let cloned = jpy.clone();
        let _usd = "1000".parse::<Money<USD>>().unwrap();

        assert_eq!(jpy, cloned);
        // assert_ne!(jpy, _usd); // expected struct `money::JPY`, found struct `money::USD`
    }
}
