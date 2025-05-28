use std::collections::HashSet;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

use merkle::{calc_root, hash_val, hex_to_bytes32, verify};

fn diff() {
    let raw = Command::new("git")
        .args(["show", "origin/main:data.txt"])
        .output()
        .unwrap()
        .stdout;

    let old_users: Vec<String> = String::from_utf8(raw)
        .unwrap()
        .lines()
        .map(|s| str::trim(s).to_string())
        .collect();

    // Load current data
    let raw = fs::read_to_string("data.txt").unwrap();
    let new_users: Vec<String> = raw.lines().map(|s| str::trim(s).to_string()).collect();

    assert!(
        new_users.len() == old_users.len() + 1,
        "Invalid number of users old: {:#?} new: {:#?}",
        old_users,
        new_users,
    );
}

fn main() {
    diff();
    return;

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

    let src = File::open(proof_path).unwrap();
    let reader = BufReader::new(src);

    let mut proof: Vec<[u8; 32]> = Vec::new();
    for line in reader.lines() {
        let v: String = line.unwrap().parse::<String>().unwrap().trim().to_string();
        proof.push(hex_to_bytes32(&v).unwrap());
    }

    let idx = hashes.len() - 1;
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
