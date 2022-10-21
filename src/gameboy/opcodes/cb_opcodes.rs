lazy_static! {
    // This comment won't be needed once the vector gets long enough
    //#[rustfmt_skip]
    pub static ref CB_DICTIONARY: Vec<(u8, Vec<&'static str>)> =
        vec![
            (0x11, "RL C"),
            (0x30, "SWAP B"),
            (0x31, "SWAP C"),
            (0x32, "SWAP D"),
            (0x33, "SWAP E"),
            (0x34, "SWAP H"),
            (0x35, "SWAP L"),
            (0x36, "SWAP (HL)"),
            (0x37, "SWAP A"),
            (0x7C, "BIT 7 H"),
        ]
            .iter()
            .map(|(i, s)| (*i, s.split(' ').collect::<Vec<&'static str>>()))
            .collect();
}
