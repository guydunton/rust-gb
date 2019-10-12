#[derive(Debug, Copy, Clone)]
pub enum Catagory {
    NOP,
    LD16,
    LD8,
    XOR,
    BIT,
    JR,
    INC,
    CALL,
    PUSH,
    RL,
    RLA,
}

fn is_cb_catagory(catagory: Catagory) -> bool {
    match catagory {
        Catagory::RL => true,
        Catagory::BIT => true,
        _ => false,
    }
}

pub fn catagory_from_str(cat: &str) -> Catagory {
    match cat {
        "NOP" => Catagory::NOP,
        "LD16" => Catagory::LD16,
        "LD8" => Catagory::LD8,
        "XOR" => Catagory::XOR,
        "BIT" => Catagory::BIT,
        "JR" => Catagory::JR,
        "INC" => Catagory::INC,
        "CALL" => Catagory::CALL,
        "PUSH" => Catagory::PUSH,
        "RL" => Catagory::RL,
        _ => Catagory::NOP,
    }
}

pub fn catagory_size(catagory: Catagory) -> u16 {
    let cb_size = if is_cb_catagory(catagory) { 1 } else { 0 };
    let total_size = cb_size + 1;
    total_size
}
