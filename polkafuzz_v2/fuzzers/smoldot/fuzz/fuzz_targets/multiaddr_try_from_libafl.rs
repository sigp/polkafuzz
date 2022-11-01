#![no_main]
use cargo_libafl_helper::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() > 0 {
        smoldot_lib::smoldot_multiaddr_try_from(data);
    }
});
