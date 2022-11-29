use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let smoldot_res = smoldot::smoldot_multiaddr_try_from(data);
            let substrate_res = substrate::substrate_multiaddr_try_from(data);
            let gossamer_res = gossamer::gossamer_new_multiaddr_bytes(data);
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
