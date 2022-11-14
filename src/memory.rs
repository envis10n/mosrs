/**
A trait defining reading and writing from a 16-bit addressed memory controller.

* This can be used for anything that should be able to be read from or written to.
*/
pub trait Memory {
    /// Read an 8-bit value from the provided address.
    fn read(&self, address: u16) -> u8;
    /// Write an 8-bit value to the provided address.
    fn write(&mut self, address: u16, value: u8);
    /// Read a 16-bit value from the provided address.
    /// 
    /// Default implementation uses LE byte order, and the implemented `Memory::read()` trait method.
    fn read_u16(&self, address: u16) -> u16 {
        let low_byte = self.read(address) as u16;
        let high_byte = self.read(address.wrapping_add(1)) as u16;
        (high_byte << 8) | low_byte
    }
    /// Write a 16-bit value to the provided address.
    /// 
    /// Default implementation uses LE byte order, and the implemented `Memory::write()` trait method.
    fn write_u16(&mut self, address: u16, value: u16) {
        let high_byte = (value >> 8) as u8;
        let low_byte = (value & high_byte as u16) as u8;
        self.write(address, low_byte);
        self.write(address.wrapping_add(1), high_byte);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct RAM {
        memory: [u8;0xffff]
    }
    
    impl RAM {
        fn new() -> Self {
            RAM { memory: [0u8;0xffff]}
        }
    }
    
    impl Memory for RAM {
        fn read(&self, address: u16) -> u8 {
            self.memory[address as usize]
        }
        fn write(&mut self, address: u16, value: u8) {
            self.memory[address as usize] = value;
        }
    }
    #[test]
    fn test_read_write() {
        let mut ram = RAM::new();
        ram.write(0xff6c, 0xcc); // write an 8-bit value to 0xff6c
        ram.write_u16(0xff00, 0xff6c); // write a 16-bit value to 0xff00
        let address = ram.read_u16(0xff00); // read the address from 0xff00
        let value = ram.read(address); // read the value from 0xff6c
        assert_eq!(address, 0xff6c, "Address {:#x}", address);
        assert_eq!(value, 0xcc, "Value: {:#x}", value);
    }
}