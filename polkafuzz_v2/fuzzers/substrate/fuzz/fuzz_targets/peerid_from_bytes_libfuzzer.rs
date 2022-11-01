#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    substrate_lib::substrate_peerid_from_bytes(data);
});
