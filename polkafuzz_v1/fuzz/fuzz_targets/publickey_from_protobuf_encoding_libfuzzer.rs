#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let smoldot_res = smoldot::smoldot_publickey_from_protobuf_encoding(data);
    let substrate_res = substrate::substrate_publickey_from_protobuf_encoding(data);
    let gossamer_res = gossamer::gossamer_publickey_from_proto(data);
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
