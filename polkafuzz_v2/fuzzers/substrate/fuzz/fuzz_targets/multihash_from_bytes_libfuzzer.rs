#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() > 0 {
        substrate_lib::substrate_multihash_from_bytes(data);
    }
});
