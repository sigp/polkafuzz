pub fn smoldot_chain_spec_from_json_bytes(data: &[u8]) -> bool {
    let ret = smoldot::chain_spec::ChainSpec::from_json_bytes(data);
    if let Err(_) = ret {
        false
    } else {
        true
    }
}

pub fn smoldot_multiaddr_try_from(data: &[u8]) -> bool {
    let ret = smoldot::libp2p::multiaddr::Multiaddr::try_from(data.to_vec());
    if let Err(_) = ret {
        false
    } else {
        true
    }
}