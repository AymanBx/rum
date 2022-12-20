use std::collections::HashMap;
pub mod rumload;
pub mod rumops;

// Universal Machine Word type is defined here to be used throughout the
// biuld of the machine
pub type Umw = u32;

// Define a structure Field that describes a part of an UM instruction by 
// holding its properties
pub struct Field {
    pub width: Umw,
    pub lsb: Umw,
}

// Universal Machine structure representing all parts required to run a machine
pub struct Machine{
     pub registers: [Umw; 8],
     pub memory: HashMap<Umw, Vec<Umw>>,
     pub unmapped: Vec<Umw>,
     pub instruction_pointer: usize,
     pub seg_counter: Umw,
}

// Define values that represent each field of the um instrusction
pub static RA: Field = Field {width: 3, lsb: 6};
pub static RB: Field = Field {width: 3, lsb: 3};
pub static RC: Field = Field {width: 3, lsb: 0};
pub static RL: Field = Field {width: 3, lsb: 25};
pub static VL: Field = Field {width: 25, lsb: 0};
pub static OP: Field = Field {width: 4, lsb: 28};

pub enum Opcode { 
    CMov, SLoad, SStore, Add, Mul, Div, BNAND, HALT, MapSeg, UnMapSeg, Out, In, Lp, Lv
}