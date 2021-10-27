pub trait Entity {
    type Id;

    fn id(&self) -> Self::Id;
}
