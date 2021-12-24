pub const ARC_KONFIG: &'static str = include_str!("arc_konfig.txt");

#[no_mangle]
pub extern "C" fn kernel_config() -> &'static str {
    return ARC_KONFIG;
}
