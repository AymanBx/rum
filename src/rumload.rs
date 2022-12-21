use std::convert::TryInto;

// Universal Machine Word type is defined here to be used throughout the
// biuld of the machine
type Umw = u32;

pub fn load(input: Option<&str>) -> Vec<Umw> {
    let mut raw_reader: Box<dyn std::io::BufRead> = match input {
        None => Box::new(std::io::BufReader::new(std::io::stdin())),
        Some(filename) => Box::new(std::io::BufReader::new(std::fs::File::open(filename).unwrap(),)),
    };

    let mut buf = Vec::<u8>::new();

    raw_reader.read_to_end(&mut buf).unwrap();
    
    let instructions: Vec<Umw> = buf
        .chunks_exact(4)
        .map(|x| u32::from_be_bytes(x.try_into().unwrap()))
        .collect();
    instructions
}
