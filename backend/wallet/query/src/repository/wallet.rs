use anyhow::{Error, Result};

pub trait ListRepository<T> {
    fn list(&self) -> Result<Vec<T>, Error>;
}

pub trait GetRepository<T> {
    fn get(&self, id: String) -> Result<T, Error>;
}
