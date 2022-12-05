#[allow(clippy::upper_case_acronyms)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Category {
    NOP,
    LD16,
    LD8,
    XOR,
    BIT,
    JP,
    ADD,
    AND,
    INC,
    DEC,
    CALL,
    RET,
    RETI,
    PUSH,
    POP,
    RL,
    RLA,
    SUB,
    CP,
    OR,
    EI,
    DI,
    SWAP,
    CPL,
    SCF,
}

fn is_cb_category(category: Category) -> bool {
    matches!(category, Category::RL | Category::BIT)
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
        "AND" => Category::AND,
        "INC" => Category::INC,
        "DEC" => Category::DEC,
        "CALL" => Category::CALL,
        "RET" => Category::RET,
        "RETI" => Category::RETI,
        "PUSH" => Category::PUSH,
        "POP" => Category::POP,
        "RL" => Category::RL,
        "SUB" => Category::SUB,
        "CP" => Category::CP,
        "OR" => Category::OR,
        "EI" => Category::EI,
        "DI" => Category::DI,
        "CPL" => Category::CPL,
        "SWAP" => Category::SWAP,
        "SCF" => Category::SCF,
        _ => {
            panic!("Failed to create category {:?}", cat);
        }
    }
}

pub fn category_size(category: Category) -> u16 {
    let cb_size = if is_cb_category(category) { 1 } else { 0 };
    cb_size + 1
}
