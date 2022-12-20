use std::{env, collections::HashMap, process::exit};
use rum::{rumload,Umw, Field, Machine, rumops::*, RA, RB, RC, RL, VL, OP, Opcode};


fn mask(bits: Umw) -> Umw { (1 << bits) - 1 }

pub fn get(field: &Field, instruction: Umw) -> usize {
    ((instruction >> field.lsb) & mask(field.width)) as usize
}

///
/// Create a mutable array of (u32) and a length of 8, “registers”, all initial values are 0.
/// Annotate a mutable variable: “instruction_pointer” of type (usize) and initial value 0.
/// Annotate a mutable variable: “seg_counter” of type (usize) with initial value 1, to 
/// hold the number of segments that are currently mapped (if none were unmapped).
/// Create a vector <u32> called “unmapped” that holds segment identifier for unmapped 
/// segments, to be reused in the future.
/// Create a mutable hashmap “segments” of keys (u32) and values (Vec<u32>), 
/// initialized with the segment 0, of key 0 and a vector that holds instructions (in u32 
/// code words) found in the file “program” that was passed to the initialize function.
/// Return all the initialized values
fn initialize() -> Machine {
    let input = env::args().nth(1);
    let instructions: Vec<Umw> = rumload::load(input.as_deref());
    
    let registers: [Umw; 8] = [0;8];
    let memory: HashMap<Umw, Vec<Umw>> = HashMap::from([(0, instructions)]);
    let unmapped: Vec<Umw> = vec![];
    let instruction_pointer: usize = 0;
    let seg_counter: Umw = 1;
     
    Machine {registers, memory, unmapped, instruction_pointer, seg_counter}
}


fn main() {
    // Initialize a universal machine with the called binary
    let mut um = initialize();

    loop {
        let instruction = um.memory.get(&0).unwrap()[um.instruction_pointer];
        um.instruction_pointer += 1;

        let ra = get(&RA, instruction);
        let rb = get(&RB, instruction);
        let rc = get(&RC, instruction);
        let rl = get(&RL, instruction);
        let vl = get(&VL, instruction);
        
        match get(&OP, instruction) {
            
            o if o == Opcode::CMov as usize => { c_mov(&mut um, rc, rb, ra) }

            o if o == Opcode::SLoad as usize =>{ s_load(&mut um, rc, rb, ra) }

            o if o == Opcode::SStore as usize =>{ s_store(&mut um, rc, rb, ra) }

            o if o == Opcode::Add as usize =>{ add(&mut um, rc, rb, ra) }

            o if o == Opcode::Mul as usize =>{ mul(&mut um, rc, rb, ra) }

            o if o == Opcode::Div as usize =>{ div(&mut um, rc, rb, ra) }

            o if o == Opcode::BNAND as usize => { nand(&mut um, rc, rb, ra) }

            o if o == Opcode::HALT as usize => { exit(0) }

            o if o == Opcode::MapSeg as usize => { map(&mut um, rc, rb) }

            o if o == Opcode::UnMapSeg as usize => { un_map(&mut um, rc) }

            o if o == Opcode::Out as usize => { output_val(&um, rc) }

            o if o == Opcode::In as usize => { input_val(&mut um, rc) }            

            o if o == Opcode::Lp as usize => { jump(&mut um, rc, rb) }

            o if o == Opcode::Lv as usize => { loadv(&mut um, rl, vl) }

            _ => { panic!("Wrong instruction!") }
        }
    }    
}