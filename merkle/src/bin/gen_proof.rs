use ethers::types::H256;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use merkle::{calc_root, gen_proof, hash_val, verify};

fn main() {
    let args: Vec<String> = env::args().collect();

    let src_path = &args[1];
    let dst_path = &args[2];

    let src = File::open(src_path).unwrap();
    let reader = BufReader::new(src);

    let mut hashes: Vec<[u8; 32]> = Vec::new();
    for line in reader.lines() {
        let v: String = line.unwrap().parse::<String>().unwrap().trim().to_string();
        hashes.push(hash_val(&v));
    }

    let idx = hashes.len() - 1;
    let root = calc_root(&hashes);
    let proof = gen_proof(&hashes, idx);

    let r: H256 = root.into();
    println!("root {:#x}", r);

    let leaf: H256 = hashes[idx].into();
    println!("leaf {:#x}", leaf);

    for p in proof.iter() {
        let h: H256 = p.into();
        println!("proof {:#x}", h);
    }

    let is_valid = verify(root, &proof, leaf.into());
    println!("{:?}", is_valid);

    let dst = File::create(dst_path).unwrap();
    let mut writer = BufWriter::new(dst);

    writeln!(writer, "{:#x}", r).unwrap();
    writeln!(writer, "{:#x}", leaf).unwrap();

    for p in proof {
        let h: H256 = p.into();
        writeln!(writer, "{:#x}", h).unwrap();
    }

    println!("proof saved to {}", dst_path);
}
