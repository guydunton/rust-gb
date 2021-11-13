#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Category {
    NOP,
    LD16,
    LD8,
    XOR,
    BIT,
    JP,
    ADD,
    INC,
    DEC,
    CALL,
    RET,
    PUSH,
    POP,
    RL,
    RLA,
    SUB,
    CP,
    OR,
    EI,
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
        "JP" => Category::JP,
        "ADD" => Category::ADD,
        "INC" => Category::INC,
        "DEC" => Category::DEC,
        "CALL" => Category::CALL,
        "RET" => Category::RET,
        "PUSH" => Category::PUSH,
        "POP" => Category::POP,
        "RL" => Category::RL,
        "SUB" => Category::SUB,
        "CP" => Category::CP,
        "OR" => Category::OR,
        "EI" => Category::EI,
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
