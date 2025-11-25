use winit::event_loop::EventLoop;

mod clib;
mod cxx;
mod dbus;
mod window;

#[derive(Clone, Debug)]
pub struct AppInfo {
    title: String,
    model: String,
    location: String,
    format: String,
    is_online: bool,
}

impl AppInfo {
    fn new() -> Self {
        AppInfo {
            title: "Rust for Aurora OS".to_string(),
            model: dbus::dbus_deviceinfo::device_info_get_device_model().unwrap(),
            location: clib::c_appdir::get_app_cache_location().unwrap(),
            format: cxx::rust::cxx_format::time("yyyy-mm-ddThh:mm:ssZ"),
            is_online: cxx::rust::cxx_qnetwork::is_online(),
        }
    }
}

fn main() {
    // Run window
    window::app_winit::main::show(AppInfo::new(), EventLoop::new().unwrap());
}
