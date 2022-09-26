pub fn smoldot_chain_spec_from_json_bytes(data: &[u8]) -> bool {
    let ret = smoldot::chain_spec::ChainSpec::from_json_bytes(data);
    if let Err(_) = ret {
        false
    } else {
        true
    }
}
