#![no_main]
use cargo_libafl_helper::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() == 32 {
        substrate_lib::substrate_publickey_from_protobuf_encoding(data);
    }    
});