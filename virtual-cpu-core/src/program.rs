use crate::memory::Memory;
use crate::stack::Stack;

use core::ops::Add;

pub trait Program {
    type Address: Copy + Add<u8, Output = Self::Address>;
    type Mem: Memory<Address = Self::Address>;
    type Stk: Stack<Mem = Self::Mem>;

    fn get_pc(&self) -> Self::Address;
    fn get_instruction(&self, m: &Self::Mem) -> &[u8];

    fn advance(&mut self);

    fn jump(&mut self, addr: Self::Address);
    fn call(&mut self, m: &mut Self::Mem, s: &mut Self::Stk, addr: Self::Address);
    fn ret(&mut self, m: &mut Self::Mem, s: &mut Self::Stk);
}
