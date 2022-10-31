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

pub fn smoldot_decode_babepredigest(data: &[u8]) -> bool {
    let ret = smoldot::header::BabePreDigestRef::from_slice(data);
    if let Err(_) = ret {
        false
    } else {
        true
    }
}

pub fn smoldot_publickey_from_protobuf_encoding(data: &[u8]) -> bool {
    let ret = smoldot::libp2p::peer_id::PublicKey::from_protobuf_encoding(data);
    if let Err(_) = ret {
        false
    } else {
        true
    }
}

pub fn smoldot_peerid_from_bytes(data: &[u8]) -> bool {
    let ret = smoldot::libp2p::peer_id::PeerId::from_bytes(data.to_vec());
    if let Err(_) = ret {
        false
    } else {
        true
    }
}

pub fn smoldot_multihash_from_bytes(data: &[u8]) -> bool {
    let ret = smoldot::libp2p::multihash::MultihashRef::from_bytes(data);
    if let Err(_) = ret {
        false
    } else {
        true
    }
}

pub fn smoldot_decode_babenextepoch(data: &[u8]) -> bool {
    let ret = smoldot::header::BabeNextEpochRef::from_slice(data);
    if let Err(_) = ret {
        false
    } else {
        true
    }
}
