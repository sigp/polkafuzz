#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() > 0 {
        smoldot_lib::smoldot_multiaddr_from_str(data);
    }
});
