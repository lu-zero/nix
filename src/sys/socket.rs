use std::{mem, ptr};
use libc::{c_int, socklen_t};
use fcntl::{Fd, fcntl, F_SETFL, F_SETFD, FD_CLOEXEC, O_NONBLOCK};
use errno::{SysResult, SysError, from_ffi};
use features;

pub use libc::{in_addr, sockaddr_in, sockaddr_in6, sockaddr_un, sa_family_t};

pub use self::consts::*;

mod ffi {
    use libc::{c_int, c_void, socklen_t};
    pub use libc::{socket, listen, bind, accept, connect, setsockopt};

    extern {
        pub fn getsockopt(
            sockfd: c_int,
            level: c_int,
            optname: c_int,
            optval: *mut c_void,
            optlen: *mut socklen_t) -> c_int;
    }
}

// Extra flags - Supported by Linux 2.6.27, normalized on other platforms
bitflags!(
    flags SockFlag: c_int {
        const SOCK_NONBLOCK = 0o0004000,
        const SOCK_CLOEXEC  = 0o2000000
    }
)

pub enum SockAddr {
    SockIpV4(sockaddr_in),
    SockIpV6(sockaddr_in6),
    SockUnix(sockaddr_un)
}

#[cfg(target_os = "linux")]
mod consts {
    use libc::{c_int};

    pub type AddressFamily = c_int;

    pub const AF_UNIX: AddressFamily  = 1;
    pub const AF_LOCAL: AddressFamily = AF_UNIX;
    pub const AF_INET: AddressFamily  = 2;
    pub const AF_INET6: AddressFamily = 10;

    pub type SockType = c_int;

    pub const SOCK_STREAM: SockType = 1;
    pub const SOCK_DGRAM: SockType = 2;
    pub const SOCK_SEQPACKET: SockType = 5;
    pub const SOCK_RAW: SockType = 3;
    pub const SOCK_RDM: SockType = 4;

    pub type SockLevel = c_int;

    pub const SOL_IP: SockLevel     = 0;
    pub const SOL_SOCKET: SockLevel = 1;
    pub const SOL_TCP: SockLevel    = 6;
    pub const SOL_UDP: SockLevel    = 17;
    pub const SOL_IPV6: SockLevel   = 41;

    pub type SockOpt = c_int;

    pub const SO_ACCEPTCONN: SockOpt = 30;
    pub const SO_BINDTODEVICE: SockOpt = 25;
    pub const SO_BROADCAST: SockOpt = 6;
    pub const SO_BSDCOMPAT: SockOpt = 14;
    pub const SO_DEBUG: SockOpt = 1;
    pub const SO_DOMAIN: SockOpt = 39;
    pub const SO_ERROR: SockOpt = 4;
    pub const SO_DONTROUTE: SockOpt = 5;
    pub const SO_KEEPALIVE: SockOpt = 9;
    pub const SO_LINGER: SockOpt = 13;
    pub const SO_MARK: SockOpt = 36;
    pub const SO_OOBINLINE: SockOpt = 10;
    pub const SO_PASSCRED: SockOpt = 16;
    pub const SO_PEEK_OFF: SockOpt = 42;
    pub const SO_PEERCRED: SockOpt = 17;
    pub const SO_PRIORITY: SockOpt = 12;
    pub const SO_PROTOCOL: SockOpt = 38;
    pub const SO_RCVBUF: SockOpt = 8;
    pub const SO_RCVBUFFORCE: SockOpt = 33;
    pub const SO_RCVLOWAT: SockOpt = 18;
    pub const SO_SNDLOWAT: SockOpt = 19;
    pub const SO_RCVTIMEO: SockOpt = 20;
    pub const SO_SNDTIMEO: SockOpt = 21;
    pub const SO_REUSEADDR: SockOpt = 2;
    pub const SO_RXQ_OVFL: SockOpt = 40;
    pub const SO_SNDBUF: SockOpt = 7;
    pub const SO_SNDBUFFORCE: SockOpt = 32;
    pub const SO_TIMESTAMP: SockOpt = 29;
    pub const SO_TYPE: SockOpt = 3;
    pub const SO_BUSY_POLL: SockOpt = 46;
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod consts {
    use libc::{c_int};

    pub type AddressFamily = c_int;

    pub const AF_UNIX: AddressFamily  = 1;
    pub const AF_LOCAL: AddressFamily = AF_UNIX;
    pub const AF_INET: AddressFamily  = 2;
    pub const AF_INET6: AddressFamily = 30;

    pub type SockType = c_int;

    pub const SOCK_STREAM: SockType = 1;
    pub const SOCK_DGRAM: SockType = 2;
    pub const SOCK_SEQPACKET: SockType = 5;
    pub const SOCK_RAW: SockType = 3;
    pub const SOCK_RDM: SockType = 4;

    pub type SockLevel = c_int;

    pub const SOL_SOCKET: SockLevel = 0xffff;

    pub type SockOpt = c_int;

    pub const SO_ACCEPTCONN: SockOpt          = 0x0002;
    pub const SO_BROADCAST: SockOpt           = 0x0020;
    pub const SO_DEBUG: SockOpt               = 0x0001;
    pub const SO_DONTTRUNC: SockOpt           = 0x2000;
    pub const SO_ERROR: SockOpt               = 0x1007;
    pub const SO_DONTROUTE: SockOpt           = 0x0010;
    pub const SO_KEEPALIVE: SockOpt           = 0x0008;
    pub const SO_LABEL: SockOpt               = 0x1010;
    pub const SO_LINGER: SockOpt              = 0x0080;
    pub const SO_NREAD: SockOpt               = 0x1020;
    pub const SO_NKE: SockOpt                 = 0x1021;
    pub const SO_NOSIGPIPE: SockOpt           = 0x1022;
    pub const SO_NOADDRERR: SockOpt           = 0x1023;
    pub const SO_NOTIFYCONFLICT: SockOpt      = 0x1026;
    pub const SO_NP_EXTENSIONS: SockOpt       = 0x1083;
    pub const SO_NWRITE: SockOpt              = 0x1024;
    pub const SO_OOBINLINE: SockOpt           = 0x0100;
    pub const SO_PEERLABEL: SockOpt           = 0x1011;
    pub const SO_RCVBUF: SockOpt              = 0x1002;
    pub const SO_RCVLOWAT: SockOpt            = 0x1004;
    pub const SO_SNDLOWAT: SockOpt            = 0x1003;
    pub const SO_RCVTIMEO: SockOpt            = 0x1006;
    pub const SO_SNDTIMEO: SockOpt            = 0x1005;
    pub const SO_RANDOMPORT: SockOpt          = 0x1082;
    pub const SO_RESTRICTIONS: SockOpt        = 0x1081;
    pub const SO_RESTRICT_DENYIN: SockOpt     = 0x00000001;
    pub const SO_RESTRICT_DENYOUT: SockOpt    = 0x00000002;
    pub const SO_REUSEADDR: SockOpt           = 0x0004;
    pub const SO_REUSESHAREUID: SockOpt       = 0x1025;
    pub const SO_SNDBUF: SockOpt              = 0x1001;
    pub const SO_TIMESTAMP: SockOpt           = 0x0400;
    pub const SO_TIMESTAMP_MONOTONIC: SockOpt = 0x0800;
    pub const SO_TYPE: SockOpt                = 0x1008;
    pub const SO_WANTMORE: SockOpt            = 0x4000;
    pub const SO_WANTOOBFLAG: SockOpt         = 0x8000;
    #[allow(type_overflow)]
    pub const SO_RESTRICT_DENYSET: SockOpt    = 0x80000000;
}

pub fn socket(domain: AddressFamily, mut ty: SockType, flags: SockFlag) -> SysResult<Fd> {
    let feat_atomic = features::socket_atomic_cloexec();

    if feat_atomic {
        ty = ty | flags.bits();
    }

    // TODO: Check the kernel version
    let res = unsafe { ffi::socket(domain, ty, 0) };

    if res < 0 {
        return Err(SysError::last());
    }

    if !feat_atomic {
        if flags.contains(SOCK_CLOEXEC) {
            try!(fcntl(res, F_SETFD(FD_CLOEXEC)));
        }

        if flags.contains(SOCK_NONBLOCK) {
            try!(fcntl(res, F_SETFL(O_NONBLOCK)));
        }
    }

    Ok(res)
}

pub fn listen(sockfd: Fd, backlog: uint) -> SysResult<()> {
    let res = unsafe { ffi::listen(sockfd, backlog as c_int) };
    from_ffi(res)
}

pub fn bind(sockfd: Fd, addr: &SockAddr) -> SysResult<()> {
    let res = unsafe {
        match *addr {
            SockIpV4(ref addr) => ffi::bind(sockfd, mem::transmute(addr), mem::size_of::<sockaddr_in>() as socklen_t),
            SockIpV6(ref addr) => ffi::bind(sockfd, mem::transmute(addr), mem::size_of::<sockaddr_in6>() as socklen_t),
            SockUnix(ref addr) => ffi::bind(sockfd, mem::transmute(addr), mem::size_of::<sockaddr_un>() as socklen_t)
        }
    };

    from_ffi(res)
}

pub fn accept(sockfd: Fd) -> SysResult<Fd> {
    let res = unsafe { ffi::accept(sockfd, ptr::null_mut(), ptr::null_mut()) };

    if res < 0 {
        return Err(SysError::last());
    }

    Ok(res)
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
pub fn accept4(sockfd: Fd, flags: SockFlag) -> SysResult<Fd> {
    use libc::sockaddr;

    type F = unsafe extern "C" fn(c_int, *mut sockaddr, *mut socklen_t, c_int) -> c_int;

    extern {
        #[linkage = "extern_weak"]
        static accept4: *const ();
    }

    if !accept4.is_null() {
        let res = unsafe {
            mem::transmute::<*const (), F>(accept4)(
                sockfd, ptr::null_mut(), ptr::null_mut(), flags.bits)
        };

        if res < 0 {
            return Err(SysError::last());
        }

        Ok(res)
    } else {
        accept4_polyfill(sockfd, flags)
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub fn accept4(sockfd: Fd, flags: SockFlag) -> SysResult<Fd> {
    accept4_polyfill(sockfd, flags)
}

#[inline]
fn accept4_polyfill(sockfd: Fd, flags: SockFlag) -> SysResult<Fd> {
    let res =  unsafe { ffi::accept(sockfd, ptr::null_mut(), ptr::null_mut()) };

    if res < 0 {
        return Err(SysError::last());
    }

    if flags.contains(SOCK_CLOEXEC) {
        try!(fcntl(res, F_SETFD(FD_CLOEXEC)));
    }

    if flags.contains(SOCK_NONBLOCK) {
        try!(fcntl(res, F_SETFL(O_NONBLOCK)));
    }

    Ok(res)
}

pub fn connect(sockfd: Fd, addr: &SockAddr) -> SysResult<()> {
    let res = unsafe {
        match *addr {
            SockIpV4(ref addr) => ffi::connect(sockfd, mem::transmute(addr), mem::size_of::<sockaddr_in>() as socklen_t),
            SockIpV6(ref addr) => ffi::connect(sockfd, mem::transmute(addr), mem::size_of::<sockaddr_in6>() as socklen_t),
            SockUnix(ref addr) => ffi::connect(sockfd, mem::transmute(addr), mem::size_of::<sockaddr_un>() as socklen_t)
        }
    };

    from_ffi(res)
}

#[repr(C)]
pub struct linger {
    pub l_onoff: c_int,
    pub l_linger: c_int
}

pub fn getsockopt<T>(fd: Fd, level: SockLevel, opt: SockOpt, val: &mut T) -> SysResult<uint> {
    let mut len = mem::size_of::<T>() as socklen_t;

    let res = unsafe {
        ffi::getsockopt(
            fd, level, opt,
            mem::transmute(val),
            &mut len as *mut socklen_t)
    };

    if res < 0 {
        return Err(SysError::last());
    }

    Ok(len as uint)
}

pub fn setsockopt<T>(fd: Fd, level: SockLevel, opt: SockOpt, val: &T) -> SysResult<()> {
    let len = mem::size_of::<T>() as socklen_t;

    let res = unsafe {
            ffi::setsockopt(
            fd, level, opt,
            mem::transmute(val),
            len)
    };

    from_ffi(res)
}