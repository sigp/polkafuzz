#[link(name = "gfuzz", kind = "static")]
extern "C" {
    pub fn gfuzz_genesis_json_from_bytes(
        data_ptr: *mut u8,
        data_size: usize,
    ) -> bool;

    pub fn gfuzz_new_multiaddr_bytes(
        data_ptr: *mut u8,
        data_size: usize,
    ) -> bool;

    pub fn gfuzz_new_multiaddr(
        data_ptr: *mut u8,
        data_size: usize,
    ) -> bool;

    pub fn gfuzz_decode_babepredigest(
        data_ptr: *mut u8,
        data_size: usize,
    ) -> bool;

    pub fn gfuzz_publickey_from_proto(
        data_ptr: *mut u8,
        data_size: usize,
    ) -> bool;

    pub fn gfuzz_peerid_from_bytes(
        data_ptr: *mut u8,
        data_size: usize,
    ) -> bool;

    pub fn gfuzz_multihash_from_bytes(
        data_ptr: *mut u8,
        data_size: usize,
    ) -> bool;

    pub fn gfuzz_decode_babenextepoch(
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

pub fn gossamer_new_multiaddr_bytes(data: &[u8]) -> bool {
    let mut dt: Vec<u8> = data.into();
    let data_ptr: *mut u8 = dt.as_mut_ptr();
    let data_size: usize = data.len() as usize;
    unsafe { gfuzz_new_multiaddr_bytes(data_ptr, data_size) }
}

pub fn gossamer_new_multiaddr(data: &[u8]) -> bool {
    let mut dt: Vec<u8> = data.into();
    let data_ptr: *mut u8 = dt.as_mut_ptr();
    let data_size: usize = data.len() as usize;
    unsafe { gfuzz_new_multiaddr(data_ptr, data_size) }
}

pub fn gossamer_decode_babepredigest(data: &[u8]) -> bool {
    let mut dt: Vec<u8> = data.into();
    let data_ptr: *mut u8 = dt.as_mut_ptr();
    let data_size: usize = data.len() as usize;
    unsafe { gfuzz_decode_babepredigest(data_ptr, data_size) }   
}

pub fn gossamer_publickey_from_proto(data: &[u8]) -> bool {
    let mut dt: Vec<u8> = data.into();
    let data_ptr: *mut u8 = dt.as_mut_ptr();
    let data_size: usize = data.len() as usize;
    unsafe { gfuzz_publickey_from_proto(data_ptr, data_size)  }

}

pub fn gossamer_peerid_from_bytes(data: &[u8]) -> bool {
    let mut dt: Vec<u8> = data.into();
    let data_ptr: *mut u8 = dt.as_mut_ptr();
    let data_size: usize = data.len() as usize;
    unsafe { gfuzz_peerid_from_bytes(data_ptr, data_size) }
}

pub fn gossamer_multihash_from_bytes(data: &[u8]) -> bool {
    let mut dt: Vec<u8> = data.into();
    let data_ptr: *mut u8 = dt.as_mut_ptr();
    let data_size: usize = data.len() as usize;
    unsafe { gfuzz_multihash_from_bytes(data_ptr, data_size) }
}


pub fn gossamer_decode_babenextepoch(data: &[u8]) -> bool {
    let mut dt: Vec<u8> = data.into();
    let data_ptr: *mut u8 = dt.as_mut_ptr();
    let data_size: usize = data.len() as usize;
    unsafe { gfuzz_decode_babenextepoch(data_ptr, data_size) }   
}
