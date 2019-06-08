pub fn shift_reg_and_flag(register: u8, carry: bool) -> (u8, bool) {
    let mask = 0b1000_0000;
    let reg_contents = register;
    let eighth_bit = (reg_contents & mask) >> 7;

    let carry_flag = carry;

    // Create the new register value
    let new_register = (reg_contents << 1) | (carry_flag as u8);

    (new_register, eighth_bit == 1)
}
