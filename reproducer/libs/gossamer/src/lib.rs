use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::Read;

#[link(name = "glib", kind = "static")]
extern "C" {
    pub fn glib_chain_spec_from_json_bytes(data_ptr: *mut u8, data_size: usize);
    pub fn glib_multiaddr_from_str(data_ptr: *mut u8, data_size: usize);
    pub fn glib_multiaddr_try_from(data_ptr: *mut u8, data_size: usize);
    pub fn glib_multihash_from_bytes(data_ptr: *mut u8, data_size: usize);
    pub fn glib_decode_babepredigest(data_ptr: *mut u8, data_size: usize);
    pub fn glib_publickey_from_protobuf_encoding(data_ptr: *mut u8, data_size: usize);
    pub fn glib_peerid_from_bytes(data_ptr: *mut u8, data_size: usize);
    pub fn glib_decode_babenextepoch(data_ptr: *mut u8, data_size: usize);
}

fn read_bytes(f_name: &String) -> Result<Vec<u8>, Error> {
    let f = File::open(f_name)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn gossamer_chain_spec_from_json_bytes(file_name: &String) {
    let mut buf = read_bytes(file_name).unwrap();
    let data_ptr: *mut u8 = buf.as_mut_ptr();
    let data_size: usize = buf.len() as usize;
    unsafe { glib_chain_spec_from_json_bytes(data_ptr, data_size) };
}

pub fn gossamer_multiaddr_from_str(file_name: &String) {
    let mut buf = read_bytes(file_name).unwrap();
    let data_ptr: *mut u8 = buf.as_mut_ptr();
    let data_size: usize = buf.len() as usize;
    unsafe { glib_multiaddr_from_str(data_ptr, data_size) };
}

pub fn gossamer_multiaddr_try_from(file_name: &String) {
    let mut buf = read_bytes(file_name).unwrap();
    let data_ptr: *mut u8 = buf.as_mut_ptr();
    let data_size: usize = buf.len() as usize;
    unsafe { glib_multiaddr_try_from(data_ptr, data_size) };
}

pub fn gossamer_multihash_from_bytes(file_name: &String) {
    let mut buf = read_bytes(file_name).unwrap();
    let data_ptr: *mut u8 = buf.as_mut_ptr();
    let data_size: usize = buf.len() as usize;
    unsafe { glib_multihash_from_bytes(data_ptr, data_size) };
}

pub fn gossamer_decode_babepredigest(file_name: &String) {
    let mut buf = read_bytes(file_name).unwrap();
    let data_ptr: *mut u8 = buf.as_mut_ptr();
    let data_size: usize = buf.len() as usize;
    unsafe { glib_decode_babepredigest(data_ptr, data_size) };
}

pub fn gossamer_publickey_from_protobuf_encoding(file_name: &String) {
    let mut buf = read_bytes(file_name).unwrap();
    let data_ptr: *mut u8 = buf.as_mut_ptr();
    let data_size: usize = buf.len() as usize;
    unsafe { glib_publickey_from_protobuf_encoding(data_ptr, data_size) };
}

pub fn gossamer_peerid_from_bytes(file_name: &String) {
    let mut buf = read_bytes(file_name).unwrap();
    let data_ptr: *mut u8 = buf.as_mut_ptr();
    let data_size: usize = buf.len() as usize;
    unsafe { glib_peerid_from_bytes(data_ptr, data_size) };
}

pub fn gossamer_decode_babenextepoch(file_name: &String) {
    let mut buf = read_bytes(file_name).unwrap();
    let data_ptr: *mut u8 = buf.as_mut_ptr();
    let data_size: usize = buf.len() as usize;
    unsafe { glib_decode_babenextepoch(data_ptr, data_size) };
}
