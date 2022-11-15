use super::memory::Memory;
use super::status::StatusFlags;
use enumflags2::BitFlags;
use std::sync::{Arc, RwLock};

/**
 * A struct defining a MOS6502 and its features / operation.
 */
pub struct MOS6502<'a> {
    /// An `Arc<RwLock<dyn Memory>>` that allows mutable, multithread access to the provided memory controller.
    memory_controller: Arc<RwLock<dyn Memory + 'a>>,
    /// CPU status register.
    status: BitFlags<StatusFlags>,
}

impl<'a> MOS6502<'a> {
    pub fn new<T>(controller: Arc<RwLock<T>>) -> Self where T: Memory + 'a {
        MOS6502 {
            memory_controller: controller.clone(),
            status: StatusFlags::new(),
        }
    }
}

impl<'a> Memory for MOS6502<'a> {
    fn read(&self, address: u16) -> u8 {
        let reader = self.memory_controller.read().unwrap();
        (*reader).read(address)
    }
    fn write(&mut self, address: u16, value: u8) {
        let mut writer = self.memory_controller.write().unwrap();
        (*writer).write(address, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test Memory Controller
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
    fn test_memory_controller<'a>() {
        let ram = Arc::new(RwLock::new(RAM::new()));
        let mut cpu: MOS6502 = MOS6502::new(ram);
        cpu.write(0xff6c, 0xcc);
        cpu.write_u16(0xff00, 0xff6c);
        let offset = cpu.read_u16(0xff00);
        let value = cpu.read(offset);
        assert_eq!(offset, 0xff6c);
        assert_eq!(value, 0xcc);
    }

}