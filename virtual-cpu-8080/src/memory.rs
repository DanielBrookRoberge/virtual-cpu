use virtual_cpu_core::memory::Memory;
use std::fmt;

pub struct Memory8080 {
    m: [u8; 65536],
}

impl Memory8080 {
    pub fn new() -> Memory8080 {
        Memory8080 { m: [0; 65536] }
    }
}

impl Default for Memory8080 {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory for Memory8080 {
    type Address = u16;

    fn get_byte(&self, addr: u16) -> u8 {
        self.m[addr as usize]
    }

    fn set_byte(&mut self, addr: u16, val: u8) {
        self.m[addr as usize] = val;
    }

    fn load(&mut self, base: u16, data: &[u8]) {
        let addr = base as usize;
        self.m[addr..(addr + data.len())].copy_from_slice(data);
    }

    fn view(&self, start: u16, end: u16) -> &[u8] {
        &self.m[(start as usize)..=(end as usize)]
    }
}

impl fmt::Debug for Memory8080 {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}
