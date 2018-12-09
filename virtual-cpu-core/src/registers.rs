pub trait Registers8 {
    type Name: Copy;

    fn get8(&self, reg: Self::Name) -> u8;
    fn set8(&mut self, reg: Self::Name, val: u8);
    fn update8(&mut self, reg: Self::Name, updater: impl Fn(u8) -> u8) {
        self.set8(reg, updater(self.get8(reg)));
    }
}

pub trait Registers16 {
    type Name: Copy;

    fn get16(&self, reg: Self::Name) -> u16;
    fn set16(&mut self, reg: Self::Name, val: u16);
    fn update16(&mut self, reg: Self::Name, updater: impl Fn(u16) -> u16) {
        self.set16(reg, updater(self.get16(reg)));
    }
}
