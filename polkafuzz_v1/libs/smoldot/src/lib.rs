pub mod smoldot;

pub fn smoldot_chain_spec_from_json_bytes(data: &[u8]) -> bool {
    self::smoldot::smoldot_chain_spec_from_json_bytes(data)
}

pub fn smoldot_multiaddr_from_str(data: &[u8]) -> bool {
    self::smoldot::smoldot_multiaddr_from_str(data)
}

pub fn smoldot_multiaddr_try_from(data: &[u8]) -> bool {
    self::smoldot::smoldot_multiaddr_try_from(data)
}

pub fn smoldot_decode_babepredigest(data: &[u8]) -> bool {
    self::smoldot::smoldot_decode_babepredigest(data)
}

pub fn smoldot_publickey_from_protobuf_encoding(data: &[u8]) -> bool {
    self::smoldot::smoldot_publickey_from_protobuf_encoding(data)
}

pub fn smoldot_peerid_from_bytes(data: &[u8]) -> bool {
    self::smoldot::smoldot_peerid_from_bytes(data)
}

pub fn smoldot_multihash_from_bytes(data: &[u8]) -> bool {
    self::smoldot::smoldot_multihash_from_bytes(data)
}

pub fn smoldot_decode_babenextepoch(data: &[u8]) -> bool {
    self::smoldot::smoldot_decode_babenextepoch(data)
}
