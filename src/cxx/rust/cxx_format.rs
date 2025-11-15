#[cxx::bridge(namespace = "com::keygenqt::aurora_rs")]
mod ffi {
    unsafe extern "C++" {
        include!("aurora-rs/src/cxx/cpp/format/cxx_format.h");

        type CxxFormat;

        fn new_format() -> UniquePtr<CxxFormat>;
        fn time(self: Pin<&mut CxxFormat>, data: String) -> String;
    }
}

/// Example execute
pub fn time(data: &str) -> String {
    ffi::new_format().pin_mut().time(data.to_string())
}