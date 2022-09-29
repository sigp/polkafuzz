#![no_main]
use cargo_libafl_helper::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() > 0 {
        let smoldot_res = smoldot::smoldot_multiaddr_from_str(data);
        let substrate_res = substrate::substrate_multiaddr_from_str(data);
        let gossamer_res = gossamer::gossamer_new_multiaddr(data);
        if (smoldot_res, substrate_res) != (smoldot_res, gossamer_res) {
                panic!("
                    Smoldot Result = {:?}
                    Substrate Result = {:?}
                    Gossamer Result = {:?}",
                    smoldot_res, substrate_res, gossamer_res);
        }
    }
});