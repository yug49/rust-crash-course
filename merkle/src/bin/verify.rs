use std::collections::HashSet;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

use merkle::{calc_root, hash_val, hex_to_bytes32, verify};

fn check_diff() {
    // Get old values
    let raw = Command::new("git")
        .args(["show", "origin/main:merkle/data.txt"])
        .output()
        .unwrap()
        .stdout;

    let old_vals: Vec<String> = String::from_utf8(raw)
        .unwrap()
        .lines()
        .map(|s| str::trim(s).to_string())
        .collect();

    // Get new values
    let raw = fs::read_to_string("data.txt").unwrap();
    let new_vals: Vec<String> = raw.lines().map(|s| str::trim(s).to_string()).collect();

    // Check new value was added
    assert_eq!(
        new_vals.len(),
        old_vals.len() + 1,
        "Invalid number of users old: {:#?} new: {:#?}",
        old_vals,
        new_vals,
    );

    // Check old values are the same
    for i in 0..old_vals.len() {
        assert_eq!(
            old_vals[i], new_vals[i],
            "old != new {} {} {}",
            i, old_vals[i], new_vals[i]
        )
    }

    // Check new value is not empty
    let new_val = new_vals[new_vals.len() - 1].clone();
    assert!(!new_val.trim().is_empty(), "new value is empty");

    // Check unique
    let set: HashSet<&String> = new_vals.iter().collect();
    assert_eq!(set.len(), new_vals.len(), "duplicate users");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let data_path = &args[1];
    let proof_path = &args[2];
    let skip_diff = &args.get(3);

    if let Some(s) = skip_diff {
        if *s == "diff" {
            check_diff();
        }
    }

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
