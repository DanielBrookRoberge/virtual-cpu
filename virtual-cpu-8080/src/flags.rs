use virtual_cpu_core::flags::Flags;

#[derive(PartialEq, Debug, Default)]
pub struct Flags8080 {
    pub z: bool,
    pub s: bool,
    pub p: bool,
    pub cy: bool,
    pub ac: bool,
}

impl Flags8080 {
    pub fn new() -> Flags8080 {
        Default::default()
    }

    pub fn set_z(&mut self, n: u8) {
        self.z = n == 0;
    }

    pub fn set_s(&mut self, n: u8) {
        self.s = (n & 0x80) != 0;
    }

    pub fn set_p(&mut self, n: u8) {
        self.p = (n.count_ones() & 0x01) == 0;
    }
}

impl Flags for Flags8080 {
    type Representation = u8;

    fn serialize(&self) -> u8 {
        (self.z as u8)
            | (self.s as u8) << 1
            | (self.p as u8) << 2
            | (self.cy as u8) << 3
            | (self.ac as u8) << 4
    }

    fn deserialize(&mut self, flags: u8) {
        self.z = (flags & 0x01) != 0;
        self.s = (flags & 0x02) != 0;
        self.p = (flags & 0x04) != 0;
        self.cy = (flags & 0x08) != 0;
        self.ac = (flags & 0x10) != 0;
    }
}
