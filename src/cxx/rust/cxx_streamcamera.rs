#[cxx::bridge(namespace = "com::keygenqt::aurora_rs")]
mod ffi {
    unsafe extern "C++" {
        include!("aurora-rs/src/cxx/cpp/streamcamera/cxx_streamcamera.h");

        type CxxStreamCamera;

        fn new_streamcamera() -> UniquePtr<CxxStreamCamera>;
        fn count_cameras(&self) -> u64;
    }
}

/// Example execute
pub fn count_cameras() -> u64 {
    ffi::new_streamcamera().count_cameras()
}