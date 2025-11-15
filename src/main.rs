mod clib;
mod cxx;
mod dbus;

fn main() {
    println!("--------------");
    println!("| D-Bus       > Device model: {:?}", dbus::dbus_deviceinfo::ac_device_info_get_device_model());
    println!("| Clang, Lib  > Cache location: {:?}", clib::c_appdir::get_app_cache_location());
    println!("| C++, Clean  > Format time: {:?}", cxx::rust::cxx_format::time("yyyy-mm-ddThh:mm:ssZ"));
    println!("| C++, Lib    > Count cameras: {:?}", cxx::rust::cxx_streamcamera::count_cameras());
    println!("| C++, Qt     > Is online:: {:?}", cxx::rust::cxx_qnetwork::is_online());
    println!("--------------");
}