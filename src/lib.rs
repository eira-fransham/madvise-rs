extern crate libc;

use std::io;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum AccessPattern {
    Normal = libc::MADV_NORMAL,
    Sequential = libc::MADV_SEQUENTIAL,
    Random = libc::MADV_RANDOM,
    DontNeed = libc::MADV_DONTNEED,
    WillNeed = libc::MADV_WILLNEED,
}

/// Advise the operating system on the access pattern of this data. On unix-like systems this can be
/// used to allow the operating system to page memory in and out more efficiently. This is used
/// extensively by allocators, and will only improve performance in real programs in rare
/// circumstances.
///
/// On Windows and other non-unix systems this is compiled into a no-op. As far as I can tell
/// Windows has no equivalent API.
///
/// ## Example
///
/// ```rust
/// use madvise::{AccessPattern, AdviseMemory};
/// let mut my_vec = vec![0; 1024];
/// my_vec.advise_memory_access(AccessPattern::Sequential).expect("Advisory failed");
/// for i in &mut my_vec {
///     *i += 1;
/// }
/// ```
pub trait AdviseMemory {
    fn advise_memory_access(&self, advice: AccessPattern) -> io::Result<()>;
}

/// Raw advise wrapper with proper error handling. This enforces the same contract as
/// `libc::madvise`. Specifically, `ptr` must be non-null and the data between `ptr` and `ptr + len`
/// must be initialized.
///
/// ## Example
///
/// ```rust
/// use madvise::{AccessPattern, madvise};
/// use std::mem;
///
/// struct BigStruct([u8; 1024]);
///
/// let mut heap_allocated_big_struct = Box::new(BigStruct([0; 1024]));
/// unsafe {
///     madvise(
///         Box::as_ref(&heap_allocated_big_struct) as *const BigStruct as *const u8,
///         mem::size_of::<BigStruct>(),
///         AccessPattern::Sequential
///     ).expect("Advisory failed");
/// }
///
/// for i in heap_allocated_big_struct.0.iter_mut() {
///     *i += 1
/// }
/// ```
pub unsafe fn madvise(ptr: *const u8, len: usize, advice: AccessPattern) -> io::Result<()> {
    let result = libc::madvise(ptr as *mut libc::c_void, len, advice as libc::c_int);

    if result == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

#[cfg(unix)]
impl AdviseMemory for [u8] {
    fn advise_memory_access(&self, advice: AccessPattern) -> io::Result<()> {
        unsafe { madvise(self.as_ptr(), self.len(), advice) }
    }
}

#[cfg(not(unix))]
impl AdviseMemory for [u8] {
    fn advise_memory_access(&self, _: AccessPattern) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
