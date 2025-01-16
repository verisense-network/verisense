use alloc::format;
use core::str::FromStr;

use ethabi::{Address, Token};
use frame_support::IterableStorageMap;
use serde::{Deserialize, Serialize};
use sp_core::{Bytes, keccak_256, U256};
use sp_core::bounded::alloc;
use sp_std::collections::btree_map::BTreeMap;
use sp_std::vec;
use sp_std::vec::Vec;

use vrs_primitives::RewardsProof;

use crate::{Config, Pallet};
use crate::pallet::{RewardsRoot, TotalRewards};
use crate::String;

impl<T: Config> Pallet<T> {
    pub fn calculate_rewards_root() {
        let mut leaves: Vec<Vec<String>> = vec![];
        for (k, v) in  <TotalRewards<T> as IterableStorageMap<T::AccountId, u128>>::iter() {
            let source = Self::validator_source(&k);
            leaves.push(vec![source.0, source.1, format!("{v}")])
        }
        let m = StandardMerkleTree::of(leaves, &vec!["address".parse().unwrap(), "string".parse().unwrap(), "uint256".parse().unwrap()]);
        RewardsRoot::<T>::put(m.root());
    }

    pub fn get_reward_proof(acct: T::AccountId) -> RewardsProof {
        let reward_amt = Self::total_rewards(acct.clone());
        if reward_amt == 0 {
            return RewardsProof::default();
        }
        let mut leaves: Vec<Vec<String>> = vec![];
        let mut  reward_index = 0usize;
        let mut index = 0usize;

        for (k, v) in  <TotalRewards<T> as IterableStorageMap<T::AccountId, u128>>::iter() {
            if k == acct {
                reward_index = index;
            }
            let source = Self::validator_source(&k);
            leaves.push(vec![source.0, source.1, format!("{v}")]);
            index += 1;
        }

        let m = StandardMerkleTree::of(leaves, &vec!["address".parse().unwrap(), "string".parse().unwrap(), "uint256".parse().unwrap()]);
        let proof = m.get_proof(LeafType::Number(reward_index));

        RewardsProof {
            proof,
            amount: format!("{reward_amt}")
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Values {
    value: Vec<String>,
    tree_index: usize,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StandardMerkleTree {
    hash_lookup: BTreeMap<String, usize>,
    tree: Vec<Bytes>,
    values: Vec<Values>,
    leaf_encoding: Vec<String>,
}

#[allow(dead_code)]
struct HashedValues {
    value: Vec<String>,
    value_index: usize,
    hash: Bytes,
}

pub enum LeafType {
    Number(usize),
}

impl StandardMerkleTree {

    fn new(tree: Vec<Bytes>, values: &[Values], leaf_encode: &[String]) -> Self {
        let mut hash_lookup = BTreeMap::new();
        values.iter().enumerate().for_each(|(i, v)| {
            hash_lookup.insert(
                hex::encode(standard_leaf_hash(v.value.clone(), leaf_encode).to_vec()),
                i,
            );
        });

        Self {
            hash_lookup,
            tree,
            values: values.to_vec(),
            leaf_encoding: leaf_encode.to_vec(),
        }
    }

    fn validate_value(&self, index: usize) {
        check_bounds(&self.values, index);
        let value = self.values.get(index).unwrap();
        check_bounds(&self.tree, value.tree_index);
        let leaf = standard_leaf_hash(value.value.clone(), &self.leaf_encoding);
        if !leaf.eq(self.tree.get(value.tree_index).unwrap()) {
            panic!("Merkle tree does not contain the expected value")
        }
    }

    pub fn get_proof(&self, leaf: LeafType) -> Vec<String> {
        let value_index = match leaf {
            LeafType::Number(i) => i,
        };
        self.validate_value(value_index);

        // rebuild tree index and generate proof
        let value = self.values.get(value_index).unwrap();
        let proof = get_proof(self.tree.clone(), value.tree_index);

        // check proof
        let hash = self.tree.get(value.tree_index).unwrap();
        let implied_root = process_proof(hash.clone(), &proof);

        if !implied_root.eq(self.tree.get(0).unwrap()) {
            panic!("Unable to prove value")
        }

        proof
            .iter()
            .map(|p| format!("0x{}", hex::encode(p.to_vec())))
            .collect()
    }

    pub fn root(&self) -> String {
        format!("0x{}", hex::encode(&self.tree[0].to_vec()))
    }


    pub fn of(values: Vec<Vec<String>>, leaf_encode: &[String]) -> Self {
        let mut hashed_values: Vec<HashedValues> = values
            .iter()
            .enumerate()
            .map(|(i, v)| HashedValues {
                value: (*v).to_vec(),
                value_index: i,
                hash: standard_leaf_hash(v.clone(), leaf_encode),
            })
            .collect();

        hashed_values.sort_by(|a, b| a.hash.cmp(&b.hash));

        let tree = make_merkle_tree(hashed_values.iter().map(|v| v.hash.clone()).collect());

        let mut indexed_values: Vec<Values> = values
            .iter()
            .map(|v| Values {
                value: (*v).to_vec(),
                tree_index: 0,
            })
            .collect();
        hashed_values.iter().enumerate().for_each(|(i, v)| {
            indexed_values[v.value_index].tree_index = tree.len() - i - 1;
        });

        Self::new(tree, &indexed_values, leaf_encode)
    }
}

pub fn standard_leaf_hash(values: Vec<String>, types: &[String]) -> Bytes {
    let mut tokens: Vec<Token> = Vec::new();
    for (i, t) in types.iter().enumerate() {
        match t.as_str() {
            "address" => {
                let address = Address::from_str(&values[i]).unwrap();
                tokens.push(Token::Address(address));
            }
            "uint" | "uint256" => {
                let uint = U256::from_dec_str(&values[i]).unwrap();
                tokens.push(Token::Uint(uint));
            }
            "string" => {
                tokens.push(Token::String(values[i].clone()));
            }
            _ => panic!("Invalid type"),
        }
    }
    let r = ethabi::encode(&tokens);
    Bytes::from(keccak_256(keccak_256(r.as_slice()).as_slice()).to_vec())
}

pub fn make_merkle_tree(leaves: Vec<Bytes>) -> Vec<Bytes> {
    if leaves.is_empty() {
        panic!("Expected non-zero number of leaves")
    };

    let tree_length = 2 * leaves.len() - 1;
    let mut tree: Vec<Bytes> = vec![Bytes::from([0].to_vec()); tree_length];

    leaves
        .iter()
        .enumerate()
        .for_each(|(i, v)| tree[tree_length - 1 - i] = v.clone());

    for i in (0..tree_length - leaves.len()).rev() {
        let left_child = tree[left_child_index(i)].clone();
        let right_child = &tree[right_child_index(i)];
        tree[i] = hash_pair(left_child, right_child);
    }

    tree
}

pub fn hash_pair(a: Bytes, b: &Bytes) -> Bytes {
    let mut s = [a, b.clone()];
    s.sort();
    let r = [s[0].to_vec(),s[1].to_vec()].concat();
    Bytes::from(keccak_256(r.as_slice()).to_vec())
}


pub fn left_child_index(i: usize) -> usize {
    2 * i + 1
}

pub fn right_child_index(i: usize) -> usize {
    2 * i + 2
}

pub fn get_proof(tree: Vec<Bytes>, mut i: usize) -> Vec<Bytes> {
    check_leaf_node(&tree, i);

    let mut proof = Vec::new();

    while i > 0 {
        let sibling_i = sibling_index(i.try_into().unwrap()).unwrap();
        proof.push(tree[sibling_i].clone());
        i = parent_index(i).unwrap();
    }
    proof
}

pub fn parent_index(i: usize) -> Result<usize, String> {
    if i > 0 {
        Ok((i - 1) / 2)
    } else {
        Err("Root has no parent".parse().unwrap())
    }
}

pub fn sibling_index(i: i32) -> Result<usize,String> {
    if i > 0 {
        let r = i - (-1i32).pow((i % 2).try_into().unwrap());
        Ok(r as usize)
    } else {
        Err("Root has no sibling".parse().unwrap())
    }
}

pub fn process_proof(leaf: Bytes, proof: &[Bytes]) -> Bytes {
    check_valid_merkle_node(&leaf);

    proof.iter().for_each(check_valid_merkle_node);

    proof.iter().fold(leaf, hash_pair)
}

pub fn is_valid_merkle_node(node: &Bytes) -> bool {
    node.len() == 32
}

pub fn check_valid_merkle_node(node: &Bytes) {
    if !is_valid_merkle_node(node) {
        panic!("Index is not in tree")
    }
}

pub fn check_leaf_node(tree: &[Bytes], i: usize) {
    if !is_leaf_node(tree, i) {
        panic!("Index is not in tree");
    }
}

pub fn is_leaf_node(tree: &[Bytes], i: usize) -> bool {
    is_tree_node(tree, i) && !is_internal_node(tree, i)
}


pub fn is_tree_node(tree: &[Bytes], i: usize) -> bool {
    i < tree.len()
}

pub fn check_bounds<T>(values: &[T], index: usize) {
    if index >= values.len() {
        panic!("Index out of range")
    }
}

pub fn is_internal_node(tree: &[Bytes], i: usize) -> bool {
    is_tree_node(tree, left_child_index(i))
}

#[test]
pub fn test1() {
    let value = vec![
        vec!["0x3390594CD564C24349e0A59e5e9b7b4645DB327f".to_string(), "10202".to_string(), "nbc".to_string()],
        vec!["0x3390594CD564C24349e0A59e5e9b7b4645DB327f".to_string(), "10202".to_string(), "nbc".to_string()],
        vec!["0x3390594CD564C24349e0A59e5e9b7b4645DB327f".to_string(), "10202".to_string(), "nbc".to_string()],
        vec!["0x3390594CD564C24349e0A59e5e9b7b4645DB327f".to_string(), "10202".to_string(), "nbc".to_string()],
        vec!["0x3390594CD564C24349e0A59e5e9b7b4645DB327f".to_string(), "10202".to_string(), "nbc".to_string()],
        vec!["0x3390594CD564C24349e0A59e5e9b7b4645DB327f".to_string(), "10202".to_string(), "nbc".to_string()],
        vec!["0x3390594CD564C24349e0A59e5e9b7b4645DB327f".to_string(), "10202".to_string(), "nbc".to_string()],
        vec!["0x3390594CD564C24349e0A59e5e9b7b4645DB327f".to_string(), "10202".to_string(), "nbc".to_string()],
        vec!["0x3390594CD564C24349e0A59e5e9b7b4645DB327f".to_string(), "10202".to_string(), "nbc".to_string()],
    ];
    let leaf_encode = vec!["address".to_string(), "uint256".to_string(), "string".to_string()];
    let r = StandardMerkleTree::of(value, &leaf_encode);
    println!("{}", r.root());
}


/*
#[test]
pub fn yre() {
    let v = vec![keccak_256("wefweryhtasd".as_bytes()),
                             keccak_256("wereshbdxfasd".as_bytes()),
                             keccak_256("wefarhesrhsrfhdgsd".as_bytes()),
                             keccak_256("weree4erffasd".as_bytes()),
                             keccak_256("wefweryhtasd".as_bytes()),
                             keccak_256("wereshbdxfasd".as_bytes()),
                             keccak_256("wefarhesrhsrfhdgsd".as_bytes()),
                             keccak_256("weree4erffasd".as_bytes()),
                        ];
    let m: MerkleTree<sha3::Sha3_256> = merkle_lite::MerkleTree::from_iter(v.clone());
    assert_eq!(
    m. proof(&[1,4,3])
        .unwrap()
        .verify(&[

            (4, v[4]),
            (1, v[1]),
            (3, v[3]),
        ])
        .unwrap()
        .as_ref(),
    m. root().unwrap(),
);
    let r = m.proof(&[2]);
    let m = hex::encode(m.root().unwrap());
    println!("{m}")
}*/

