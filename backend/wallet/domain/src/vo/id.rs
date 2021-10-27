use crate::vo::Identifier;
use crate::vo::ValueObject;
use anyhow::Error;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::str::FromStr;

#[derive(Debug, Default, Clone, Eq)]
pub struct Id<E>(String, PhantomData<E>);

impl<E> ValueObject for Id<E> {}

impl<E> Identifier for Id<E> {}

impl<E> FromStr for Id<E> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string(), PhantomData))
    }
}

impl<T> ToString for Id<T> {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Entity {}

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct AnotherEntity {}

    #[derive(Clone, Debug, PartialEq, Eq)]
    enum EnumEntity {}

    #[test]
    fn test_id() {
        assert!("01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id::<Entity>>().is_ok());
        assert!("01F8MECHZX3TBDSZ7XRADM79XE"
            .parse::<Id::<AnotherEntity>>()
            .is_ok());
        assert!("01F8MECHZX3TBDSZ7XRADM79XE"
            .parse::<Id::<EnumEntity>>()
            .is_ok());
    }

    #[test]
    fn test_id_to_string() {
        assert_eq!(
            "01F8MECHZX3TBDSZ7XRADM79XE"
                .parse::<Id::<Entity>>()
                .unwrap()
                .to_string(),
            "01F8MECHZX3TBDSZ7XRADM79XE".to_string()
        );
    }

    #[test]
    fn test_id_eq() {
        let id = "01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id<Entity>>().unwrap();
        let cloned = id.clone();

        let another = "01F8MECHZX3TBDSZ7XRADM79XF".parse::<Id<Entity>>().unwrap();

        assert_eq!(id, cloned);
        assert_ne!(id, another);
    }

    #[test]
    fn test_id_hash() {
        let id = "01F8MECHZX3TBDSZ7XRADM79XE".parse::<Id<Entity>>().unwrap();
        let entity = Entity {};
        let mut m = HashMap::new();

        m.insert(id.clone(), entity);

        assert!(m.get(&id).is_some());
    }
}
