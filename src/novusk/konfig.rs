#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn kernel_config() -> &'static str {
    include_str!("quantii_config.txt")
}
