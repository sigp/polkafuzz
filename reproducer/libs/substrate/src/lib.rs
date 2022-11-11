use codec::Decode;
use sc_chain_spec::GenericChainSpec;
use serde::{Deserialize, Serialize};
use sp_consensus_babe::digests::{NextEpochDescriptor, PreDigest};
use sp_runtime::generic::Header;
use sp_runtime::traits::BlakeTwo256;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::Read;
use std::str::FromStr;

fn read_bytes(f_name: &String) -> Result<Vec<u8>, Error> {
    let f = File::open(f_name)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn substrate_chain_spec_from_json_bytes(file_name: &String) {
    println!("[+] Substrate Result:");
    let buf = read_bytes(file_name).unwrap();
    #[derive(Debug, Serialize, Deserialize)]
    struct Genesis(BTreeMap<String, String>);
    let ret = GenericChainSpec::<Genesis>::from_json_bytes(Cow::Owned(buf));
    if let Err(e) = ret {
        println!("[-] ChainSpec from_json_bytes result: {:?}", e);
    } else {
        println!("[+] ChainSpec from_json_bytes result: Ok()");
    }
}

pub fn substrate_multiaddr_from_str(file_name: &String) {
    println!("[+] Substrate Result:");
    let buf = read_bytes(file_name).unwrap();
    let buf_str = String::from_utf8_lossy(&buf);
    let vec: Vec<&str> = buf_str.split("\n").collect();
    let ret = multiaddr::Multiaddr::from_str(vec[0]);
    if let Err(_) = ret {
        println!("[-] Multiaddr from_str result: {:?}", ret);
    } else {
        println!("[+] Multiaddr from_str result: {:?}", ret);
    }
}

pub fn substrate_multiaddr_try_from(file_name: &String) {
    println!("[+] Substrate Result:");
    let buf = read_bytes(file_name).unwrap();
    let ret = multiaddr::Multiaddr::try_from(buf);
    if let Err(_) = ret {
        println!("[-] Multiaddr try_from  result: {:?}", ret);
    } else {
        println!("[+] Multiaddr try_from  result: {:?}", ret);
    }
}

pub fn substrate_multihash_from_bytes(file_name: &String) {
    println!("[+] Substrate Result:");
    let buf = read_bytes(file_name).unwrap();
    let ret = multihash::Multihash::from_bytes(&buf);
    if let Err(_) = ret {
        println!("[-] Multihash from_bytes result: {:?}", ret);
    } else {
        println!("[+] Multihash from_bytes result: {:?}", ret);
    }
}

pub fn substrate_decode_babepredigest(file_name: &String) {
    println!("[+] Substrate Result:");
    let buf = read_bytes(file_name).unwrap();
    let mut data: &[u8] = buf.as_ref();
    let ret = PreDigest::decode(&mut data);
    if let Err(_) = ret {
        println!("[-] BabePredigest decode result: {:?}", ret);
    } else {
        println!("[+] BabePredigest decode result: {:?}", ret);
    }
}

pub fn substrate_publickey_from_protobuf_encoding(file_name: &String) {
    println!("[+] Substrate Result:");
    let buf = read_bytes(file_name).unwrap();
    let ret = libp2p::identity::PublicKey::from_protobuf_encoding(&buf);
    if let Err(_) = ret {
        println!("[-] PublicKey from_protobuf_encoding result: {:?}", ret);
    } else {
        println!("[+] PublicKey from_protobuf_encoding result: {:?}", ret);
    }
}

pub fn substrate_peerid_from_bytes(file_name: &String) {
    println!("[+] Substrate Result:");
    let buf = read_bytes(file_name).unwrap();
    let ret = libp2p::core::PeerId::from_bytes(&buf);
    if let Err(_) = ret {
        println!("[-] PeerId from_bytes result: {:?}", ret);
    } else {
        println!("[+] PeerId from_bytes result: {:?}", ret);
    }
}

pub fn substrate_decode_babenextepoch(file_name: &String) {
    println!("[+] Substrate Result:");
    let buf = read_bytes(file_name).unwrap();
    let mut data: &[u8] = buf.as_ref();
    let ret = NextEpochDescriptor::decode(&mut data);
    if let Err(_) = ret {
        println!("[-] BabeNextEpoch decode result: {:?}", ret);
    } else {
        println!("[+] BabeNexetEpoch decode result: {:?}", ret);
    }
}

pub fn substrate_decode_header(file_name: &String) {
    println!("[+] Substrate Result:");
    let buf = read_bytes(file_name).unwrap();
    let mut data: &[u8] = buf.as_ref();
    let ret = Header::<u32, BlakeTwo256>::decode(&mut data);
    if let Err(_) = ret {
        println!("[-] Header decode result: {:?}", ret);
    } else {
        println!("[+] Header decode result: {:?}", ret);
    }
}
