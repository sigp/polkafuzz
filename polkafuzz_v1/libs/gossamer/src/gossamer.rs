#[link(name = "gfuzz", kind = "static")]
extern "C" {
    pub fn gfuzz_genesis_json_from_bytes(
        data_ptr: *mut u8,
        data_size: usize,
    ) -> bool;

    pub fn gfuzz_new_multiaddr(
        data_ptr: *mut u8,
        data_size: usize,
    ) -> bool;

}

pub fn gossamer_genesis_json_from_bytes(data: &[u8]) -> bool {
    let mut dt: Vec<u8> = data.into();
    let data_ptr: *mut u8 = dt.as_mut_ptr();
    let data_size: usize = data.len() as usize;
    unsafe { gfuzz_genesis_json_from_bytes(data_ptr, data_size) }
}

pub fn gossamer_new_multiaddr(data: &[u8]) -> bool {
    let mut dt: Vec<u8> = data.into();
    let data_ptr: *mut u8 = dt.as_mut_ptr();
    let data_size: usize = data.len() as usize;
    unsafe { gfuzz_new_multiaddr(data_ptr, data_size) }
}