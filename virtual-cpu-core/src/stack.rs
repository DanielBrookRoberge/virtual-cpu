use crate::memory::Memory;

use std::ops::Add;

pub trait Stack {
    type Address: Add<Self::Address, Output = Self::Address> + Copy + From<u8>;
    type Mem: Memory<Address = Self::Address>;

    fn get_sp(&self) -> Self::Address;
    fn set_sp(&mut self, val: Self::Address);

    fn pop_byte(&mut self, m: &mut Self::Mem) -> u8;
    fn push_byte(&mut self, m: &mut Self::Mem, val: u8);

    fn pop_word(&mut self, m: &mut Self::Mem) -> u16;
    fn push_word(&mut self, m: &mut Self::Mem, val: u16);
}
