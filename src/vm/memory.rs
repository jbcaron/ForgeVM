use super::error::{Result, VmError};

/// The memory structure used by the VM.
/// The memory has a fixed size and can store any type.
/// The memory is byte-addressable.
/// The memory is cleared to zero when created.
/// The memory can be read from and written to.
/// The memory access must be aligned to the size of the type.
/// The memory access must be within the bounds of the memory.
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    /// Create a new memory with the specified size.
    /// 
    /// # Parameters
    /// - `size`: The size of the memory in bytes.
    pub fn new(size: usize) -> Self {
        Memory {
            data: vec![0; size],
        }
    }

    /// Clear the memory by setting all values to zero.
    pub fn clear(&mut self) {
        self.data.iter_mut().for_each(|x| *x = 0);
    }

    /// Read a value from memory at the specified address.
    /// The address must be aligned to the size of the type `T`.
    /// 
    /// # Parameters
    /// - `address`: The address to read from.
    /// 
    /// # Returns
    /// The value read from memory.
    /// 
    /// # Errors
    /// Returns an error if the address is out of bounds or not aligned.
    pub fn read<T>(&self, address: usize) -> Result<T>
    where
        T: Copy,
    {
        if address + std::mem::size_of::<T>() > self.data.len() {
            return Err(VmError::MemoryOutOfBounds {
                address,
                size: std::mem::size_of::<T>(),
            });
        } else if address % std::mem::align_of::<T>() != 0 {
            return Err(VmError::MemoryNotAligned {
                address,
                size: std::mem::size_of::<T>(),
            });
        }

        Ok(unsafe { *(self.data.as_ptr().add(address) as *const T) })
    }

    /// Write a value to memory at the specified address.
    /// The address must be aligned to the size of the type `T`.
    /// 
    /// # Parameters
    /// - `address`: The address to write to.
    /// - `value`: The value to write.
    /// 
    /// # Errors
    /// Returns an error if the address is out of bounds or not aligned.
    pub fn write<T>(&mut self, address: usize, value: T) -> Result<()> {
        if address + std::mem::size_of::<T>() > self.data.len() {
            return Err(VmError::MemoryOutOfBounds {
                address,
                size: std::mem::size_of::<T>(),
            });
        } else if address % std::mem::align_of::<T>() != 0 {
            return Err(VmError::MemoryNotAligned {
                address,
                size: std::mem::size_of::<T>(),
            });
        }

        unsafe {
            *(self.data.as_mut_ptr().add(address) as *mut T) = value;
        }

        Ok(())
    }

    /// Get the capacity of the memory.
    pub fn capacity(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_create() {
        let memory = Memory::new(1024);
        assert_eq!(memory.data.len(), 1024);
    }

    #[test]
    fn test_memory_read_write() {
        let mut memory = Memory::new(1024);

        memory.write::<u32>(0, 0x12345678).unwrap();
        assert_eq!(memory.read::<u32>(0).unwrap(), 0x12345678);
    }

    #[test]
    fn test_memory_out_of_bounds_u8() {
        let mut memory = Memory::new(1024);

        assert!(memory.read::<u8>(1023).is_ok());
        assert!(memory.read::<u8>(1024).is_err());
        assert!(memory.write::<u8>(1023, 0x12).is_ok());
        assert!(memory.write::<u8>(1024, 0x34).is_err());
    }

    #[test]
    fn test_memory_out_of_bounds_u32() {
        let mut memory = Memory::new(1024);

        assert!(memory.read::<u32>(1020).is_ok());
        assert!(memory.read::<u32>(1021).is_err());
        assert!(memory.write::<u32>(1020, 0x12345678).is_ok());
        assert!(memory.write::<u32>(1021, 0x12345678).is_err());
    }

    #[test]
    fn test_memory_not_aligned_u16() {
        let mut memory = Memory::new(1024);

        assert!(memory.read::<u16>(0).is_ok());
        assert!(memory.read::<u16>(1).is_err());
        assert!(memory.write::<u16>(0, 0x1234).is_ok());
        assert!(memory.write::<u16>(1, 0x1234).is_err());
    }
}
