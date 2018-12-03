use virtual_cpu_core::bytes::*;
use virtual_cpu_core::{Memory, Stack};

use crate::memory::Memory8080;

#[derive(Default, Debug)]
pub struct Stack8080 {
    sp: u16,
}

impl Stack8080 {
    pub fn new() -> Stack8080 {
        Stack8080::default()
    }
}

impl Stack for Stack8080 {
    type Address = u16;
    type Mem = Memory8080;

    fn get_sp(&self) -> u16 {
        self.sp
    }
    fn set_sp(&mut self, val: u16) {
        self.sp = val;
    }

    fn pop_byte(&mut self, m: &mut Memory8080) -> u8 {
        self.sp += 1;
        m.get_byte(self.sp - 1)
    }

    fn push_byte(&mut self, m: &mut Memory8080, val: u8) {
        self.sp -= 1;
        m.set_byte(self.sp, val);
    }

    fn pop_word(&mut self, m: &mut Memory8080) -> u16 {
        let low_order = self.pop_byte(m);
        let high_order = self.pop_byte(m);

        assemble_word(high_order, low_order)
    }

    fn push_word(&mut self, m: &mut Memory8080, val: u16) {
        self.push_byte(m, high_order_byte(val));
        self.push_byte(m, low_order_byte(val));
    }
}
