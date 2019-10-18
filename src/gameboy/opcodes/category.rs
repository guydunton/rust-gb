#[derive(Debug, Copy, Clone)]
pub enum Category {
    NOP,
    LD16,
    LD8,
    XOR,
    BIT,
    JR,
    INC,
    DEC,
    CALL,
    PUSH,
    POP,
    RL,
    RLA,
}

fn is_cb_category(category: Category) -> bool {
    match category {
        Category::RL => true,
        Category::BIT => true,
        _ => false,
    }
}

pub fn category_from_str(cat: &str) -> Category {
    match cat {
        "NOP" => Category::NOP,
        "RLA" => Category::RLA,
        "LD16" => Category::LD16,
        "LD8" => Category::LD8,
        "XOR" => Category::XOR,
        "BIT" => Category::BIT,
        "JR" => Category::JR,
        "INC" => Category::INC,
        "DEC" => Category::DEC,
        "CALL" => Category::CALL,
        "PUSH" => Category::PUSH,
        "POP" => Category::POP,
        "RL" => Category::RL,
        _ => {
            panic!("Failed to create category {:?}", cat);
        }
    }
}

pub fn category_size(category: Category) -> u16 {
    let cb_size = if is_cb_category(category) { 1 } else { 0 };
    let total_size = cb_size + 1;
    total_size
}
