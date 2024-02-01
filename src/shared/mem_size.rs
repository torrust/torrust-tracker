#[cfg(target_pointer_width = "64")]
pub const POINTER_SIZE: usize = 8;
#[cfg(target_pointer_width = "32")]
pub const POINTER_SIZE: usize = 4;

pub trait MemSize {
    /// Returns the memory size of a struct and all its children in bytes
    fn get_mem_size(&self) -> usize;
}
