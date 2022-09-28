pub mod substrate;

pub fn substrate_chain_spec_from_json_bytes(data: &[u8]) -> bool {
    self::substrate::substrate_chain_spec_from_json_bytes(data)
}

pub fn substrate_multiaddr_try_from(data: &[u8]) -> bool {
    self::substrate::substrate_multiaddr_try_from(data)
}
