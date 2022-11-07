use codec::Decode;
use sc_chain_spec::GenericChainSpec;
use serde::{Deserialize, Serialize};
use sp_consensus_babe::digests::{NextEpochDescriptor, PreDigest};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::str::FromStr;

pub fn substrate_chain_spec_from_json_bytes(data: &[u8]) -> bool {
    #[derive(Debug, Serialize, Deserialize)]
    struct Genesis(BTreeMap<String, String>);

    let ret = GenericChainSpec::<Genesis>::from_json_bytes(Cow::Owned(data.to_vec()));
    if let Err(_) = ret {
        false
    } else {
        true
    }
}

pub fn substrate_multiaddr_try_from(data: &[u8]) -> bool {
    let ret = multiaddr::Multiaddr::try_from(data.to_vec());
    if let Err(_) = ret {
        false
    } else {
        true
    }
}

pub fn substrate_multiaddr_from_str(data: &[u8]) -> bool {
    let data_str = String::from_utf8_lossy(data);
    let vec: Vec<&str> = data_str.split("\n").collect();
    let ret = multiaddr::Multiaddr::from_str(vec[0]);
    if let Err(_) = ret {
        false
    } else {
        true
    }
}

pub fn substrate_decode_babepredigest(mut data: &[u8]) -> bool {
    let ret = PreDigest::decode(&mut data);
    if let Err(_) = ret {
        false
    } else {
        true
    }
}

pub fn substrate_publickey_from_protobuf_encoding(data: &[u8]) -> bool {
    let ret = libp2p::identity::PublicKey::from_protobuf_encoding(data);
    if let Err(_) = ret {
        false
    } else {
        true
    }
}

pub fn substrate_peerid_from_bytes(data: &[u8]) -> bool {
    let ret = libp2p::core::PeerId::from_bytes(data);
    if let Err(_) = ret {
        false
    } else {
        true
    }
}

pub fn substrate_multihash_from_bytes(data: &[u8]) -> bool {
    let ret = multihash::Multihash::from_bytes(data);
    if let Err(_) = ret {
        false
    } else {
        true
    }
}

pub fn substrate_decode_babenextepoch(mut data: &[u8]) -> bool {
    let ret = NextEpochDescriptor::decode(&mut data);
    if let Err(_) = ret {
        false
    } else {
        true
    }
}
