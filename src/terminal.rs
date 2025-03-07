use std::io;

use libc::{STDOUT_FILENO, TIOCGWINSZ, c_ushort, ioctl};

#[repr(C)]
struct Buf {
    row: c_ushort,
    col: c_ushort,
    x_px: c_ushort,
    y_px: c_ushort,
}

type WinSize = (u32, u32);

pub fn get_terminal_size() -> io::Result<WinSize> {
    let mut buf = Buf {
        row: 0,
        col: 0,
        x_px: 0,
        y_px: 0,
    };
    match unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut buf) } {
        0 => Ok((buf.x_px.into(), buf.y_px.into())),
        _ => Err(io::Error::last_os_error()),
    }
}
