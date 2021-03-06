pub const NRBITS: u32 = 8;
pub const TYPEBITS: u32 = 8;

#[cfg(any(target_arch = "mips", target_arch = "powerpc", target_arch = "powerpc64"))]
mod consts {
    pub const NONE: u8 = 1;
    pub const READ: u8 = 2;
    pub const WRITE: u8 = 4;
    pub const SIZEBITS: u8 = 13;
    pub const DIRBITS: u8 = 3;
}

#[cfg(not(any(target_arch = "powerpc",
              target_arch = "mips",
              target_arch = "x86",
              target_arch = "arm",
              target_arch = "x86_64",
              target_arch = "powerpc64",
              target_arch = "aarch64")))]
use this_arch_not_supported;

// "Generic" ioctl protocol
#[cfg(any(target_arch = "x86",
          target_arch = "arm",
          target_arch = "x86_64",
          target_arch = "aarch64"))]
mod consts {
    pub const NONE: u8 = 0;
    pub const READ: u8 = 2;
    pub const WRITE: u8 = 1;
    pub const SIZEBITS: u8 = 14;
    pub const DIRBITS: u8 = 2;
}

pub use self::consts::*;

pub const NRSHIFT: u32 = 0;
pub const TYPESHIFT: u32 = NRSHIFT + NRBITS as u32;
pub const SIZESHIFT: u32 = TYPESHIFT + TYPEBITS as u32;
pub const DIRSHIFT: u32 = SIZESHIFT + SIZEBITS as u32;

pub const NRMASK: u32 = (1 << NRBITS) - 1;
pub const TYPEMASK: u32 = (1 << TYPEBITS) - 1;
pub const SIZEMASK: u32 = (1 << SIZEBITS) - 1;
pub const DIRMASK: u32 = (1 << DIRBITS) - 1;

/// Encode an ioctl command.
#[macro_export]
macro_rules! ioc {
    ($dir:expr, $ty:expr, $nr:expr, $sz:expr) => (
        (($dir as u32) << $crate::sys::ioctl::DIRSHIFT) |
        (($ty as u32) << $crate::sys::ioctl::TYPESHIFT) |
        (($nr as u32) << $crate::sys::ioctl::NRSHIFT) |
        (($sz as u32) << $crate::sys::ioctl::SIZESHIFT))
}

/// Encode an ioctl command that has no associated data.
#[macro_export]
macro_rules! io {
    ($ty:expr, $nr:expr) => (ioc!($crate::sys::ioctl::NONE, $ty, $nr, 0))
}

/// Encode an ioctl command that reads.
#[macro_export]
macro_rules! ior {
    ($ty:expr, $nr:expr, $sz:expr) => (ioc!($crate::sys::ioctl::READ, $ty, $nr, $sz))
}

/// Encode an ioctl command that writes.
#[macro_export]
macro_rules! iow {
    ($ty:expr, $nr:expr, $sz:expr) => (ioc!($crate::sys::ioctl::WRITE, $ty, $nr, $sz))
}

/// Encode an ioctl command that both reads and writes.
#[macro_export]
macro_rules! iorw {
    ($ty:expr, $nr:expr, $sz:expr) => (ioc!($crate::sys::ioctl::READ | $crate::sys::ioctl::WRITE, $ty, $nr, $sz))
}

/// Extracts the "direction" (read/write/none) from an encoded ioctl command.
#[inline(always)]
pub fn ioc_dir(nr: u32) -> u8 {
    ((nr >> DIRSHIFT) & DIRMASK) as u8
}

/// Extracts the type from an encoded ioctl command.
#[inline(always)]
pub fn ioc_type(nr: u32) -> u32 {
    (nr >> TYPESHIFT) & TYPEMASK
}

/// Extracts the ioctl number from an encoded ioctl command.
#[inline(always)]
pub fn ioc_nr(nr: u32) -> u32 {
    (nr >> NRSHIFT) & NRMASK
}

/// Extracts the size from an encoded ioctl command.
#[inline(always)]
pub fn ioc_size(nr: u32) -> u32 {
    ((nr >> SIZESHIFT) as u32) & SIZEMASK
}

pub const IN: u32 = (WRITE as u32) << DIRSHIFT;
pub const OUT: u32 = (READ as u32) << DIRSHIFT;
pub const INOUT: u32 = ((READ|WRITE) as u32) << DIRSHIFT;
pub const SIZE_MASK: u32 = SIZEMASK << SIZESHIFT;
