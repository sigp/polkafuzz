#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let smoldot_res = smoldot::smoldot_peerid_from_bytes(data);
    let substrate_res = substrate::substrate_peerid_from_bytes(data);
    let gossamer_res = gossamer::gossamer_peerid_from_bytes(data);
    if (smoldot_res, substrate_res) != (smoldot_res, gossamer_res) {
        panic!("
            Smoldot Result = {:?}
            Substrate Result = {:?}
            Gossamer Result = {:?}",
            smoldot_res, substrate_res, gossamer_res);
    }
});
