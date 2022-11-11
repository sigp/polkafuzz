#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() > 0 {
        let smoldot_res = smoldot::smoldot_decode_header(data);
        let substrate_res = substrate::substrate_decode_header(data);
        let gossamer_res = gossamer::gossamer_decode_header(data);
        if (smoldot_res, substrate_res) != (smoldot_res, gossamer_res) {
            panic!(
                "
                    Smoldot Result = {:?}
                    Substrate Result = {:?}
                    Gossamer Result = {:?}",
                smoldot_res, substrate_res, gossamer_res
            );
        }
    }
});
