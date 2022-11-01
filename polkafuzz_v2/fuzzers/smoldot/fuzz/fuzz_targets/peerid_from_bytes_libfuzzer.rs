#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    smoldot_lib::smoldot_peerid_from_bytes(data);
});
