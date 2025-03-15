//! Build script for getting QT version on windows

/// Outside of windows, this build script does nothing.
#[cfg(not(windows))]
fn main() {}

/// On windows, this build script will obtain the current version of QT
#[cfg(windows)]
fn main() {
    cxx_qt_build::CxxQtBuilder::new().build();
}
