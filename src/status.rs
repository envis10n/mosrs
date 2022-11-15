use enumflags2::{bitflags, BitFlags};

#[bitflags(default = UNK | Interrupt)]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StatusFlags {
    Carry = 0b00000001,
    Zero = 0b00000010,
    Interrupt = 0b000000100,
    Decimal = 0b00001000,
    Break = 0b00010000,
    UNK = 0b00100000,
    Overflow = 0b01000000,
    Negative = 0b10000000
}

impl StatusFlags {
    pub fn new() -> BitFlags<StatusFlags> {
        BitFlags::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_status() {
        let mut status = StatusFlags::new();
        status.insert(StatusFlags::Carry);
        assert!(status.contains(StatusFlags::Carry));
        assert!(status.contains(StatusFlags::UNK));
    }
}