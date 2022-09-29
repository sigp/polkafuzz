use sc_chain_spec::GenericChainSpec;
use serde::{Deserialize, Serialize};
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
    let ret = multiaddr::Multiaddr::from_str(&data_str);
    if let Err(_) = ret {
        false
    } else {
        true
    }
}