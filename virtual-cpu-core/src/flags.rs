pub trait Flags {
    type Representation;

    fn serialize(&self) -> Self::Representation;
    fn deserialize(&mut self, flags: Self::Representation);
}
