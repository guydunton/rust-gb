lazy_static! {
    //#[rustfmt_skip]
    pub static ref CB_DICTIONARY: Vec<(u8, Vec<&'static str>)> =
        vec![
            (0x11, "RL C"),
            (0x7C, "BIT 7 H"),
        ]
            .iter()
            .map(|(i, s)| (*i, s.split(' ').collect::<Vec<&'static str>>()))
            .collect();
}
