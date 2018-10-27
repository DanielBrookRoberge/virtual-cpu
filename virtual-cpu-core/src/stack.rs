use crate::memory::Memory;

pub trait Stack {
    type Mem: Memory;

    fn pop_byte(&mut self, m: &mut Self::Mem) -> u8;
    fn push_byte(&mut self, m: &mut Self::Mem, val: u8);

    fn pop_word(&mut self, m: &mut Self::Mem) -> u16;
    fn push_word(&mut self, m: &mut Self::Mem, val: u16);
}
