pub trait Registers8 {
    type Name;

    fn get8(&self, reg: Self::Name) -> u8;
    fn set8(&mut self, reg: Self::Name, val: u8);
}

pub trait Registers16 {
    type Name;

    fn get16(&self, reg: Self::Name) -> u16;
    fn set16(&mut self, reg: Self::Name, val: u16);
}
