pub mod smoldot;

pub fn smoldot_chain_spec_from_json_bytes(data: &[u8]) -> bool {
    self::smoldot::smoldot_chain_spec_from_json_bytes(data)
}
