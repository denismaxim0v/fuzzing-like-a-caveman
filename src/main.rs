use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{BufReader, Read, Write};

fn main() -> std::io::Result<()> {
    let f = File::open("./exif-samples/jpg/Canon_40D.jpg")?;

    let mut reader = BufReader::new(f);
    let mut buffer: Vec<u8> = Vec::new();

    // Read bytes to a buffer.
    reader.read_to_end(&mut buffer)?;
    let mut mutated = bit_flip(&mut buffer);
    mutated = magic_numbers(&mut mutated);
    create_new(&mutated, "test.jpg")?;

    Ok(())
}

fn create_new(data: &Vec<u8>, name: &str) -> std::io::Result<()> {
    let mut file = File::create(name)?;
    file.write_all(data)?;

    Ok(())
}

// Mutation method #1. Bit flipping.
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
            None => panic!("Couldn't choose an index."),
        }
    }

    // Flip bits.
    for i in chosen_indexes {
        let idx_range: Vec<usize> = (0..8).collect();
        let rand_bit = idx_range.choose(&mut rand::thread_rng());
        match rand_bit {
            Some(val) => {
                data[i] ^= (1 as u8) << (*val as u8);
            }
            None => panic!("Couldn't flip bit."),
        }
    }

    data
}

// Mutation method #2. Magic numbers.
fn magic_numbers(data: &Vec<u8>) -> Vec<u8> {
    let mut data = data.clone();
    let magic = [
        (1, 255),
        (1, 255),
        (1, 127),
        (1, 0),
        (2, 255),
        (2, 0),
        (4, 255),
        (4, 0),
        (4, 128),
        (4, 64),
        (4, 127),
    ];

    // Chose random magic number.
    let rand_magic = magic.choose(&mut rand::thread_rng());
    let chosen_magic = match rand_magic {
        Some(number) => *number,
        None => panic!("Coudln't choose a magic number."),
    };

    // Choose random index.
    let idxs: Vec<usize> = (2..data.len() - 2).collect();
    let chosen_idx = match idxs.choose(&mut rand::thread_rng()) {
        Some(num) => *num,
        None => panic!("Failed to choose index.")
    };

    match chosen_magic.0 {
        1 => {
            data[chosen_idx] = chosen_magic.1 as u8;
        },
        2 => {
            data[chosen_idx] = chosen_magic.1 as u8;
            data[chosen_idx + 1] = chosen_magic.1 as u8;
        },
        4 => {
            match chosen_magic.1 {
                255 => {
                    data[chosen_idx] = chosen_magic.1 as u8;
                    data[chosen_idx + 1] = chosen_magic.1 as u8;
                    data[chosen_idx + 2] = chosen_magic.1 as u8;
                    data[chosen_idx + 3] = chosen_magic.1 as u8;
                },
                0 => {
                    data[chosen_idx] = chosen_magic.1 as u8;
                    data[chosen_idx + 1] = chosen_magic.1 as u8;
                    data[chosen_idx + 2] = chosen_magic.1 as u8;
                    data[chosen_idx + 3] = chosen_magic.1 as u8;
                },
                128 => {
                    data[chosen_idx] = chosen_magic.1 as u8;
                    data[chosen_idx + 1] = 0 as u8;
                    data[chosen_idx + 2] = 0 as u8;
                    data[chosen_idx + 3] = 0 as u8;
                },
                64 => {
                    data[chosen_idx] = chosen_magic.1 as u8;
                    data[chosen_idx + 1] = 0 as u8;
                    data[chosen_idx + 2] = 0 as u8;
                    data[chosen_idx + 3] = 0 as u8;
                },
                127 => {
                    data[chosen_idx] = chosen_magic.1 as u8;
                    data[chosen_idx + 1] = 255 as u8;
                    data[chosen_idx + 2] = 255 as u8;
                    data[chosen_idx + 3] = 255 as u8;
                },
                _ => unreachable!()
            }
        },
        _ => unreachable!()
    }

    data
}
