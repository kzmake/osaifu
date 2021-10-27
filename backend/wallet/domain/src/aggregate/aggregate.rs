use crate::entity;
use crate::entity::Entity;

pub trait AggregateRoot: Entity {}

pub type Wallet = entity::Wallet;
impl AggregateRoot for Wallet {}
