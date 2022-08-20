use rand::seq::SliceRandom;
use rand::{self};
use std::fs::File;
use std::io::{BufReader, Read, Write};

fn main() -> std::io::Result<()> {
    let f = File::open("./exif-samples/jpg/Canon_40D.jpg")?;

    let mut reader = BufReader::new(f);
    let mut buffer: Vec<u8> = Vec::new();

    // Read bytes to a buffer.
    reader.read_to_end(&mut buffer)?;
    let mutated = bit_flip(&mut buffer);
    create_new(&mutated, "test.jpg")?;

    Ok(())
}

fn create_new(data: &Vec<u8>, name: &str) -> std::io::Result<()> {
    let mut file = File::create(name)?;
    file.write_all(data)?;

    Ok(())
}

fn bit_flip(data: &Vec<u8>) -> Vec<u8> {
    let number_of_flips: u64 = ((data.len() as f32 - 4 as f32) * 0.01_f32) as u64;
    let indexes: Vec<usize> = (2..(data.len() - 2)).collect();
    let mut data = data.clone();

    // Select indexes.
    let mut chosen_indexes = Vec::new();
    for _ in 0..number_of_flips {
        let idx = indexes.choose(&mut rand::thread_rng());
        match idx {
            Some(val) => chosen_indexes.push(*val),
            None => panic!("Couldn't choose an index.")
        }
    }

    // Flip bits.
    for i in chosen_indexes {
        let idx_range: Vec<usize> = (0..8).collect();
        let rand_bit = idx_range.choose(&mut rand::thread_rng());
        match rand_bit {
            Some(val) => {
                data[i] ^= (1 as u8) << (*val as u8);
            },
            None => panic!("Couldn't flip bit.")
        }
    }

    data
}
