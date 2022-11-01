#![no_main]
use cargo_libafl_helper::fuzz_target;

fuzz_target!(|data: &[u8]| {
    smoldot_lib::smoldot_chain_spec_from_json_bytes(data);
});
