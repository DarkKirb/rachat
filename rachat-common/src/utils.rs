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

    let path_buf = path.as_ref().as_os_str().encode_wide().collect::<Vec<u16>>();
    let mut out_buf = Vec::with_capacity(path_buf.len() * 2);
    for wide_char in path_buf {
        out_buf.extend_from_slice(&wide_char.to_le_bytes()[..]);
    }
    out_buf
}


#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    #[test]
    fn test_path_to_bytes() {
        let path = Path::new("/foo/bar");
        let bytes = path_to_bytes(path);
        assert_eq!(bytes, b"/foo/bar");
    }

    #[cfg(windows)]
    #[test]
    fn test_path_to_bytes() {
        let path = Path::new(r"C:\foo\bar");
        let bytes = path_to_bytes(path);
        assert_eq!(bytes, b"C\0:\0\\\0f\0o\0o\0\\\0b\0a\0r\0");
    }
}