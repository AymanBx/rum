use std::io::{stdout, Write, stdin, Read};
use crate::{Machine, Umw};


pub fn c_mov(um: &mut Machine, rc: usize, rb: usize, ra: usize){
    if um.registers[rc] != 0{
        um.registers[ra] = um.registers[rb];
    }
}

pub fn s_load(um: &mut Machine, rc: usize, rb: usize, ra: usize){
    let mem = um.memory.get(&((um.registers[rb]) as Umw)).unwrap();
    um.registers[ra] = mem[um.registers[rc] as usize];
}

pub fn s_store(um: &mut Machine, rc: usize, rb: usize, ra: usize){
    let mem = um.memory.get_mut(&((um.registers[ra]) as Umw)).unwrap();
    mem[um.registers[rb] as usize] = um.registers[rc];
}

pub fn add(um: &mut Machine, rc: usize, rb: usize, ra: usize){
    let b = um.registers[rb];
    let c = um.registers[rc];
    let a = b.wrapping_add(c);
    um.registers[ra] =  a;
}

pub fn mul(um: &mut Machine, rc: usize, rb: usize, ra: usize){
    let b = um.registers[rb];
    let c = um.registers[rc];
    let a = b.wrapping_mul(c);
    um.registers[ra] =  a;
}

pub fn div(um: &mut Machine, rc: usize, rb: usize, ra: usize){
    um.registers[ra] =
     um.registers[rb] / um.registers[rc];
}

pub fn nand(um: &mut Machine, rc: usize, rb: usize, ra: usize){
    um.registers[ra] =
     !(um.registers[rb] & um.registers[rc]);
}

pub fn map(um: &mut Machine, rc: usize, rb: usize){
    let words = um.registers[rc];
    let vals = vec![0_u32; words as usize];
    let key = um.seg_counter;
    if um.unmapped.is_empty(){
        um.memory.insert(key, vals);
        um.registers[rb] = key;
        um.seg_counter += 1;
    }
    else{
        let seg = um.unmapped.pop().unwrap();
        um.memory.insert(seg, vec![0_u32; um.registers[rc].try_into().unwrap()]);
        um.registers[rb] = seg;
    }  
}

pub fn un_map(um: &mut Machine, rc: usize){
    um.unmapped.push(um.registers[rc]);
    um.memory.remove(&um.registers[rc]).unwrap();
}

pub fn output_val(um: &Machine, rc: usize){
    let outputted_value = u8::try_from(um.registers[rc]).unwrap();
    let mut buffer = std::io::stdout();
    match buffer.write(&[outputted_value]).unwrap(){
        1 => {
            stdout().flush().unwrap();
        },
        _ => { panic!("Wrong output value")}
    }   
}

pub fn input_val(um: &mut Machine, rc: usize){
    let mut buffer = [0_u8;1];
    let mut input = stdin();
    um.registers[rc] = match input.read(&mut buffer).
    expect("Couldn't retrieve input.") {
        1 => {
            buffer[0] as u32
        }
        _ => {
            u32::MAX
        }
    }
}

pub fn jump(um: &mut Machine, rc: usize, rb: usize){
    um.instruction_pointer = um.registers[rc] as usize;
    if um.registers[rb] != 0{
        let new_prog = um.memory.get(&um.registers[rb]).unwrap().clone();
        let mem0 = um.memory.get_mut(&0).unwrap();
        *mem0 = new_prog;        
    }
}

pub fn loadv(um: &mut Machine, rl: usize, vl: usize){
    um.registers[rl] = vl as u32;
}