use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::Read;
use core::str::FromStr;

fn read_bytes(f_name: String) -> Result<Vec<u8>, Error> {
    let f = File::open(&f_name)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn smoldot_chain_spec_from_json_bytes(file_name: String) {
    let buf = read_bytes(file_name).unwrap();
    let ret = smoldot::chain_spec::ChainSpec::from_json_bytes(buf);
    if let Err(e) = ret {
        println!("[-] ChainSpec from_json_bytes result: {:?}", e);
    } else {
        println!("[+] ChainSpec from_json_bytes result: Ok()");
    }
}


pub fn smoldot_multiaddr_from_str(file_name: String) { 
    let buf = read_bytes(file_name).unwrap();
    let buf_str = String::from_utf8_lossy(&buf);
    let ret = smoldot::libp2p::multiaddr::Multiaddr::from_str(&buf_str);
    if let Err(_) = ret {
        println!("[-] Multiaddr from_str result: {:?}", ret);
    } else {
        println!("[+] Multiaddr from_str result: {:?}", ret);
    }
}

pub fn smoldot_multiaddr_try_from(file_name: String) {
    let buf = read_bytes(file_name).unwrap();
    let ret = smoldot::libp2p::multiaddr::Multiaddr::try_from(buf);
    if let Err(_) = ret {
        println!("[-] Multiaddr try_from  result: {:?}", ret);
    } else {
        println!("[+] Multiaddr try_from  result: {:?}", ret);
    }
}

pub fn smoldot_multihash_from_bytes(file_name: String) {
    let buf = read_bytes(file_name).unwrap();
    let ret = smoldot::libp2p::multihash::MultihashRef::from_bytes(&buf);
    if let Err(_) = ret {
        println!("[-] Multihash from_bytes result: {:?}", ret);
    } else {
        println!("[+] Multihash from_bytes result: {:?}", ret);
    }
}

pub fn smoldot_decode_babepredigest(file_name: String) {
    let buf = read_bytes(file_name).unwrap();
    let ret = smoldot::header::BabePreDigestRef::from_slice(&buf);
    if let Err(_) = ret {
        println!("[-] BabePredigest from_slice result: {:?}", ret);
    } else {
        println!("[+] BabePredigest from_slice result: {:?}", ret);
    }
}

pub fn smoldot_publickey_from_protobuf_encoding(file_name: String) {
    let buf = read_bytes(file_name).unwrap();
    let ret = smoldot::libp2p::peer_id::PublicKey::from_protobuf_encoding(&buf);
    if let Err(_) = ret {
        println!("[-] PublicKey from_protobuf_encoding result: {:?}", ret);
    } else {
        println!("[+] PublicKey from_protobuf_encoding result: {:?}", ret);
    }
}

pub fn smoldot_peerid_from_bytes(file_name: String) {
    let buf = read_bytes(file_name).unwrap();
    let ret = smoldot::libp2p::peer_id::PeerId::from_bytes(buf);
    if let Err(_) = ret {
        println!("[-] PeerId from_bytes result: {:?}", ret);
    } else {
        println!("[+] PeerId from_bytes result: {:?}", ret);
    }
}

pub fn smoldot_decode_babenextepoch(file_name: String) {
    let buf = read_bytes(file_name).unwrap();
    let ret = smoldot::header::BabeNextEpochRef::from_slice(&buf);
    if let Err(_) = ret {
        println!("[-] PeerId from_bytes result: {:?}", ret);
    } else {
        println!("[+] PeerId from_bytes result: {:?}", ret);
    }
}



