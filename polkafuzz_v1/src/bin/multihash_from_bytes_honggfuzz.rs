use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let smoldot_res = smoldot::smoldot_multihash_from_bytes(data);
            let substrate_res = substrate::substrate_multihash_from_bytes(data);
            let gossamer_res = gossamer::gossamer_multihash_from_bytes(data);
            if (smoldot_res, substrate_res) != (smoldot_res, gossamer_res) {
                panic!(
                    "
                        Smoldot Result = {:?}
                        Substrate Result = {:?}
                        Gossamer Result = {:?}",
                    smoldot_res, substrate_res, gossamer_res
                );
            }
        });
    }
}
