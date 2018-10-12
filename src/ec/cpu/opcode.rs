pub type OpCode = u8;

// NOTE: remember to add this value to proper *Instr array below
#[derive(FromPrimitive, Debug)]
pub enum OpCodeValue {
    Lr = 0x00,
    Str = 0x02,
    Andr = 0x04,
    Orr = 0x05,
    Xorr = 0x06,
    Notr = 0x07,
    Bcsr = 0x08,
    Ar = 0x10,
    Sr = 0x11,
    Mr = 0x13,
    Dr = 0x14,
    L = 0x20,
    St = 0x22,
    Swap = 0x23,
    And = 0x24,
    Or = 0x25,
    Xor = 0x26,
    Not = 0x27,
    Svc = 0x2e,
    A = 0x30,
    S = 0x31,
    M = 0x33,
    D = 0x34,
    Li = 0x40,
    Andi = 0x44,
    Ori = 0x45,
    Xori = 0x46,
    Noti = 0x47,
    Ai = 0x50,
    Si = 0x51,
    Mi = 0x53,
    Di = 0x54,
    Min = 0x7a,
    Max = 0x7b,
}

pub enum OpType {
    Rr,
    Rrm,
    Rs,
    Im,
}

pub static RrInstr: &'static [OpCode] = &[
    OpCodeValue::Lr as OpCode,
    OpCodeValue::Str as OpCode,
    OpCodeValue::Andr as OpCode,
    OpCodeValue::Orr as OpCode,
    OpCodeValue::Xorr as OpCode,
    OpCodeValue::Notr as OpCode,
    OpCodeValue::Ar as OpCode,
    OpCodeValue::Sr as OpCode,
    OpCodeValue::Mr as OpCode,
    OpCodeValue::Dr as OpCode,
];
pub static RrmInstr: &'static [OpCode] = &[OpCodeValue::Bcsr as OpCode];
pub static RsInstr: &'static [OpCode] = &[
    OpCodeValue::L as OpCode,
    OpCodeValue::St as OpCode,
    OpCodeValue::Swap as OpCode,
    OpCodeValue::Svc as OpCode,
    OpCodeValue::And as OpCode,
    OpCodeValue::Or as OpCode,
    OpCodeValue::Xor as OpCode,
    OpCodeValue::Not as OpCode,
    OpCodeValue::A as OpCode,
    OpCodeValue::S as OpCode,
    OpCodeValue::M as OpCode,
    OpCodeValue::D as OpCode,
    OpCodeValue::Min as OpCode,
    OpCodeValue::Max as OpCode,
];
pub static ImInstr: &'static [OpCode] = &[
    OpCodeValue::Li as OpCode,
    OpCodeValue::Andi as OpCode,
    OpCodeValue::Ori as OpCode,
    OpCodeValue::Xori as OpCode,
    OpCodeValue::Noti as OpCode,
    OpCodeValue::Ai as OpCode,
    OpCodeValue::Si as OpCode,
    OpCodeValue::Mi as OpCode,
    OpCodeValue::Di as OpCode,
];
