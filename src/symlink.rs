use std::{io, fs};
use std::path::Path;

#[cfg(windows)]
pub fn symlink(src: &Path, dst: &Path, meta: fs::Metadata) -> io::Result<()> {
    if meta.is_dir() {
        std::os::windows::fs::symlink_dir(src, dst)
    } else {
        std::os::windows::fs::symlink_file(src, dst)
    }
}

#[cfg(unix)]
pub fn symlink(src: &Path, dst: &Path, _: fs::Metadata) -> io::Result<()> {
    std::os::unix::fs::symlink(src, dst)
}
