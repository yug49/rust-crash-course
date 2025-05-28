use ethers::core::abi::encode;
use ethers::core::abi::Token;
use ethers::types::H256;
use ethers::utils::keccak256;
use hex;

mod math;
use math::min;

pub fn hash_val(val: &str) -> [u8; 32] {
    let encoded = encode(&[Token::String(val.to_string())]);
    keccak256(encoded)
}

fn hash(left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
    let mut a: H256 = left.into();
    let mut b: H256 = right.into();

    if b < a {
        (a, b) = (b, a);
    }

    let encoded = encode(&[
        Token::FixedBytes(a.as_bytes().to_vec()),
        Token::FixedBytes(b.as_bytes().to_vec()),
    ]);

    keccak256(encoded)
}

pub fn calc_root(hashes: &Vec<[u8; 32]>) -> [u8; 32] {
    let mut hs = hashes.clone();
    let mut n = hs.len();

    while n > 1 {
        for i in (0..n).step_by(2) {
            let left = hs[i];
            let right = hs[min(i + 1, n - 1)];
            hs[i / 2] = hash(left, right);
        }
        // div by 2 and round up
        // if n is even => n = n / 2
        // else         => n = (n + 1) / 2
        n = (n + (n & 1)) / 2;
    }

    hs[0]
}

pub fn gen_proof(hashes: &Vec<[u8; 32]>, idx: usize) -> Vec<[u8; 32]> {
    let mut hs = hashes.clone();
    let mut proof: Vec<[u8; 32]> = Vec::new();
    let mut n = hs.len();
    let mut k = idx;

    assert!(idx < n, "Invalid index");

    while n > 1 {
        //      1
        //    /   \
        //   2     3
        //  / \   / \
        // 4   5 6   7
        let j = if k & 1 == 1 { k - 1 } else { min(k + 1, n - 1) };
        proof.push(hs[j]);
        k /= 2;

        for i in (0..n).step_by(2) {
            let left = hs[i];
            let right = hs[min(i + 1, n - 1)];
            hs[i / 2] = hash(left, right);
        }
        // div by 2 and round up
        // if n is even => n = n / 2
        // else         => n = (n + 1) / 2
        n = (n + (n & 1)) / 2;
    }

    proof
}

pub fn verify(root: [u8; 32], proof: &[[u8; 32]], leaf_hash: [u8; 32]) -> bool {
    let mut h = leaf_hash;

    for p in proof.iter() {
        h = hash(*p, h);
    }

    h == root
}

pub fn hex_to_bytes32(hex_str: &str) -> Result<[u8; 32], String> {
    let hex_str = hex_str.strip_prefix("0x").unwrap_or(hex_str);
    let bytes = hex::decode(hex_str).map_err(|e| e.to_string())?;
    let arr: [u8; 32] = bytes.try_into().map_err(|_| "Invalid length".to_string())?;
    Ok(arr)
}
