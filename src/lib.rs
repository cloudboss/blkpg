//! A Rust library to call Linux blkpg ioctls.
//!
//! Note: only the `BLKPG_RESIZE_PARTITION` operation is implemented.

use std::ffi::{c_char, c_int, c_longlong, c_uchar, c_void};

use rustix::{
    fd::AsFd,
    io,
    ioctl::{ioctl, Direction, Ioctl, IoctlOutput, Opcode},
};

const BLK_GROUP: c_uchar = 0x12;
const BLKPG_NUM: c_uchar = 105;
const BLKPG_RESIZE_PARTITION: c_int = 0x3;

// C structs reimplemented from linux/blkpg.h.

#[repr(C)]
#[derive(Debug)]
struct BlkpgIoctlArg {
    op: c_int,
    flags: c_int,
    datalen: c_int,
    data: *mut c_void,
}

impl BlkpgIoctlArg {
    fn new(op: c_int, flags: c_int, data: *mut c_void) -> Self {
        Self {
            op,
            flags,
            // Ignored by the kernel as it does sizeof(struct blkpg_partition) to get the data length.
            datalen: 0,
            data,
        }
    }
}

unsafe impl Ioctl for BlkpgIoctlArg {
    type Output = ();

    const IS_MUTATING: bool = false;
    const OPCODE: Opcode = Opcode::from_components(Direction::None, BLK_GROUP, BLKPG_NUM, 0);

    fn as_ptr(&mut self) -> *mut c_void {
        self as *mut Self as *mut c_void
    }

    unsafe fn output_from_ptr(_: IoctlOutput, _: *mut c_void) -> io::Result<Self::Output> {
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
struct BlkpgPartition {
    start: c_longlong,
    length: c_longlong,
    pno: c_int,
    devname: [c_char; 64],
    volname: [c_char; 64],
}

impl BlkpgPartition {
    fn new(start: c_longlong, length: c_longlong, pno: c_int) -> Self {
        Self {
            start,
            length,
            pno,
            devname: [0; 64], // Ignored by the kernel.
            volname: [0; 64], // Ignored by the kernel.
        }
    }
}

/// Calls the blkpg ioctl with the `BLKPG_RESIZE_PARTITION` operation,
/// which enables resizing a partition while it is mounted.
///
/// # Arguments
///
/// * `disk_fd`: The open file descriptor of the device.
/// * `part_num`: The number of the partition to be modified.
/// * `start_sector`: The start sector of the partition.
/// * `end_sector`: The end sector of the partition.
/// * `sector_size`: The size of the sectors in bytes.
pub fn resize_partition<F: AsFd>(
    disk_fd: F,
    part_num: i32,
    start_sector: i64,
    end_sector: i64,
    sector_size: i64,
) -> io::Result<()> {
    let bp = BlkpgPartition::new(
        start_sector * sector_size,
        (end_sector - start_sector) * sector_size,
        part_num,
    );
    let arg = BlkpgIoctlArg::new(
        BLKPG_RESIZE_PARTITION,
        0,
        &bp as *const BlkpgPartition as *mut c_void,
    );
    unsafe {
        ioctl(disk_fd, arg)?;
    }
    Ok(())
}
