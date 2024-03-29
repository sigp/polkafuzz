pub fn substrate_chain_spec_from_json_bytes(data: &[u8]) -> bool {
    substrate::substrate_chain_spec_from_json_bytes(data)
}

pub fn substrate_multiaddr_from_str(data: &[u8]) -> bool {
    substrate::substrate_multiaddr_from_str(data)
}

pub fn substrate_multiaddr_try_from(data: &[u8]) -> bool {
    substrate::substrate_multiaddr_try_from(data)
}

pub fn substrate_decode_babepredigest(data: &[u8]) -> bool {
    substrate::substrate_decode_babepredigest(data)
}

pub fn substrate_publickey_from_protobuf_encoding(data: &[u8]) -> bool {
    substrate::substrate_publickey_from_protobuf_encoding(data)
}

pub fn substrate_peerid_from_bytes(data: &[u8]) -> bool {
    substrate::substrate_peerid_from_bytes(data)
}

pub fn substrate_multihash_from_bytes(data: &[u8]) -> bool {
    substrate::substrate_multihash_from_bytes(data)
}

pub fn substrate_decode_babenextepoch(data: &[u8]) -> bool {
    substrate::substrate_decode_babenextepoch(data)
}

pub fn substrate_decode_header(data: &[u8]) -> bool {
    substrate::substrate_decode_header(data)
}
