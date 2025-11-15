pub mod rust {
    pub mod cxx_format;
    #[cfg(not(target_arch = "aarch64"))]  // @todo It doesn't work yet when building for aarch64
    pub mod cxx_qnetwork;
}