use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use merkle::{calc_root, hash_val, hex_to_bytes32, verify};

fn main() {
    let args: Vec<String> = env::args().collect();

    let data_path = &args[1];
    let proof_path = &args[2];

    let src = File::open(data_path).unwrap();
    let reader = BufReader::new(src);

    let mut hashes: Vec<[u8; 32]> = Vec::new();
    for line in reader.lines() {
        let v: String = line.unwrap().parse::<String>().unwrap().trim().to_string();
        hashes.push(hash_val(&v));
    }

    let idx = hashes.len() - 1;

    let src = File::open(proof_path).unwrap();
    let reader = BufReader::new(src);

    let mut proof: Vec<[u8; 32]> = Vec::new();
    for line in reader.lines() {
        let v: String = line.unwrap().parse::<String>().unwrap().trim().to_string();
        proof.push(hex_to_bytes32(&v).unwrap());
    }

    let root = proof[0];
    let leaf = proof[1];

    assert!(
        root == calc_root(&hashes),
        "root provided as proof != calculated root"
    );
    assert!(leaf == hashes[idx], "leaf != last hash");

    let is_valid = verify(root, &proof[2..], leaf);
    println!("valid proof? {:?}", is_valid);
}
