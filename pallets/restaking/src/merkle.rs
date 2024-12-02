use merkle_lite::MerkleTree;
use sp_core::keccak_256;

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
    let m: MerkleTree<sha3::Sha3_256> = merkle_lite::MerkleTree::from_iter(v);
    let m = hex::encode(m.root().unwrap());
    println!("{m}")
}