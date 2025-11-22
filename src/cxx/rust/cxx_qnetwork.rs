#[cxx::bridge(namespace = "com::keygenqt::aurora_rs")]
mod ffi {
    unsafe extern "C++" {
        include!("aurora-rs/src/cxx/cpp/qnetwork/cxx_qnetwork.h");

        type CxxQnetwork;

        fn new_cxx_qnetwork() -> UniquePtr<CxxQnetwork>;
        fn is_online(&self) -> bool;
    }
}

/// Example execute
pub fn is_online() -> bool {
    ffi::new_cxx_qnetwork().is_online()
}
