use crate::bytes::*;

use std::ops::Add;

pub trait Memory {
    type Address: Add<Self::Address, Output = Self::Address> + Copy + From<u8>;

    // add get_dword and get_qword if necessary
    fn get_byte(&self, addr: Self::Address) -> u8;
    fn get_word(&self, addr: Self::Address) -> u16 {
        assemble_word(self.get_byte(addr + 1.into()), self.get_byte(addr))
    }

    fn set_byte(&mut self, addr: Self::Address, value: u8);
    fn set_word(&mut self, addr: Self::Address, value: u16) {
        self.set_byte(addr, low_order_byte(value));
        self.set_byte(addr + 1.into(), high_order_byte(value));
    }

    fn load(&mut self, addr: Self::Address, data: &[u8]);
    fn view(&self, start: Self::Address, end: Self::Address) -> &[u8];
}
