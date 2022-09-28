pub mod gossamer;

pub fn gossamer_genesis_json_from_bytes(data: &[u8]) -> bool {
    self::gossamer::gossamer_genesis_json_from_bytes(data)
}

pub fn gossamer_new_multiaddr(data: &[u8]) -> bool {
    self::gossamer::gossamer_new_multiaddr(data)
}