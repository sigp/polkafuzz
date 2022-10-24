pub mod gossamer;

pub fn gossamer_genesis_json_from_bytes(data: &[u8]) -> bool {
    self::gossamer::gossamer_genesis_json_from_bytes(data)
}

pub fn gossamer_new_multiaddr_bytes(data: &[u8]) -> bool {
    self::gossamer::gossamer_new_multiaddr_bytes(data)
}

pub fn gossamer_new_multiaddr(data: &[u8]) -> bool {
    self::gossamer::gossamer_new_multiaddr(data)
}

pub fn gossamer_decode_babepredigest(data: &[u8]) -> bool {
    self::gossamer::gossamer_decode_babepredigest(data)
}

pub fn gossamer_publickey_from_proto(data: &[u8]) -> bool {
    self::gossamer::gossamer_publickey_from_proto(data)
}
