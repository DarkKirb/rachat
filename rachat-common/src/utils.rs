//! miscellaneous utilities

use std::path::Path;

#[cfg(unix)]
/// Converts a path to a stable bytewise representation
pub fn path_to_bytes(path: impl AsRef<Path>) -> Vec<u8> {
    use std::os::unix::ffi::OsStrExt;

    path.as_ref().as_os_str().as_bytes().to_vec()
}

#[cfg(windows)]
/// Converts a path to a stable bytewise representation
pub fn path_to_bytes(path: impl AsRef<Path>) -> Vec<u8> {
    use safe_transmute::to_bytes::transmute_to_bytes;
    use std::os::windows::ffi::OsStrExt;

    let path_buf = path.as_os_str().encode_wide().collect::<Vec<u8>>();
    transmute_to_bytes(&path_buf[..]).to_vec()
}
