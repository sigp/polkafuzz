use std::str::FromStr;

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

pub fn smoldot_multiaddr_from_str(data: &[u8]) -> bool {
    let data_str = String::from_utf8_lossy(data);
    let ret = smoldot::libp2p::multiaddr::Multiaddr::from_str(&data_str);
    if let Err(_) = ret {
        false
    } else {
        true
    }
}