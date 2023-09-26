mod device;
mod event_handler;
#[allow(dead_code)]
mod filesystem;
pub mod fuse;
mod multikey;
mod server;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::fs_utils;
#[cfg(target_os = "linux")]
pub use linux::passthrough;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub use macos::fs_utils;
#[cfg(target_os = "macos")]
pub use macos::passthrough;

use super::bindings;
use super::descriptor_utils;

pub use self::defs::uapi::VIRTIO_ID_FS as TYPE_FS;
pub use self::device::Fs;

mod defs {
    pub const FS_DEV_ID: &str = "virtio_fs";
    pub const NUM_QUEUES: usize = 2;
    pub const QUEUE_SIZES: &[u16] = &[1024; NUM_QUEUES];

    pub mod uapi {
        /// The device conforms to the virtio spec version 1.0.
        pub const VIRTIO_F_VERSION_1: u32 = 32;
        pub const VIRTIO_ID_FS: u32 = 26;
    }
}

use std::ffi::{FromBytesWithNulError, FromVecWithNulError};
use std::io;

use descriptor_utils::Error as DescriptorError;

#[derive(Debug)]
pub enum FsError {
    /// Failed to decode protocol messages.
    DecodeMessage(io::Error),
    /// Failed to encode protocol messages.
    EncodeMessage(io::Error),
    /// Failed to create event fd.
    EventFd(std::io::Error),
    /// The guest failed to send a require extensions.
    MissingExtension,
    /// One or more parameters are missing.
    MissingParameter,
    /// A C string parameter is invalid.
    InvalidCString(FromBytesWithNulError),
    InvalidCString2(FromVecWithNulError),
    /// The `len` field of the header is too small.
    InvalidHeaderLength,
    /// The `size` field of the `SetxattrIn` message does not match the length
    /// of the decoded value.
    InvalidXattrSize((u32, usize)),
    QueueReader(DescriptorError),
    QueueWriter(DescriptorError),
}

type Result<T> = std::result::Result<T, FsError>;
