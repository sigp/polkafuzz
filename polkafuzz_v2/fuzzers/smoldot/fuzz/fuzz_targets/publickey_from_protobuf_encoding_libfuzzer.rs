#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() == 32 {
        smoldot_lib::smoldot_publickey_from_protobuf_encoding(data);
    }
});
