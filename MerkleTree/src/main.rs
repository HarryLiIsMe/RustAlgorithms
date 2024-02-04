use anyhow::{bail, Result};
use sha256::digest;
use std::{cmp::Ordering, mem::take, vec::Vec};

struct MerkleTree {
    tree: Vec<Vec<Vec<u8>>>,
}

struct MerkleProof {
    proof: Vec<Vec<u8>>,
}

impl MerkleTree {
    fn new(data: &Vec<Vec<u8>>) -> Result<Self> {
        if data.len() <= 1 {
            bail!("merkle data length less than 2!!!");
        }

        let mut tree = Vec::new();
        let mut data1 = data.clone();
        let mut data2 = Vec::new();
        let mut i = 0;
        while data1.len() > 1 {
            if i == data1.len() {
                tree.push(data1);
                data1 = take(&mut data2);
                i = 0;
                continue;
            }

            if i + 1 == data1.len() {
                let hash = if tree.is_empty() {
                    digest([&data1[i][..], &data1[i][..]].concat())
                        .as_bytes()
                        .to_owned()
                } else {
                    data1[i].clone()
                };

                data2.push(hash.clone());

                tree.push(data1);
                data1 = take(&mut data2);
                i = 0;
                continue;
            }

            let hash = if data1[i].cmp(&data1[i + 1]) == Ordering::Greater {
                digest([&data1[i][..], &data1[i + 1][..]].concat())
                    .as_bytes()
                    .to_owned()
            } else {
                digest([&data1[i + 1][..], &data1[i][..]].concat())
                    .as_bytes()
                    .to_owned()
            };

            data2.push(hash);
            i = i + 2;
        }

        tree.push(data1);
        return Ok(MerkleTree { tree });
    }

    fn get_root(&self) -> &Vec<u8> {
        return self.tree.last().unwrap().first().unwrap();
    }

    fn get_tree(&self) -> &Vec<Vec<Vec<u8>>> {
        return &self.tree;
    }

    fn get_leaf(&self) -> &Vec<Vec<u8>> {
        return self.tree.first().unwrap();
    }

    fn tree_height(&self) -> usize {
        return self.tree.len();
    }

    fn leaf_num(&self) -> usize {
        return self.tree.first().unwrap().len();
    }

    fn get_proof(&self, index: usize) -> Result<MerkleProof> {
        if index > self.tree.first().unwrap().len() - 1 {
            bail!("index greater than length of the merkle data!!!");
        }

        let mut proof = Vec::new();
        let mut index = index;

        let mut i = 0;
        while i < self.tree.len() - 1 {
            if index % 2 == 0 {
                if index == self.tree[i].len() - 1 {
                    if i == 0 {
                        proof.push(self.tree[i][index].clone());
                    }
                } else {
                    proof.push(self.tree[i][index + 1].clone());
                }
            } else {
                proof.push(self.tree[i][index - 1].clone());
            }

            index = index / 2;
            i += 1;
        }

        return Ok(MerkleProof::new(proof)?);
    }
}

impl MerkleProof {
    fn new(proof: Vec<Vec<u8>>) -> Result<Self> {
        if proof.is_empty() {
            bail!("merkle proof is empty!!!");
        }
        return Ok(MerkleProof { proof });
    }

    fn verify_proof(&self, leaf: &Vec<u8>, root: &Vec<u8>) -> bool {
        let mut hash = leaf.clone();
        for p in &self.proof {
            if p.cmp(&hash) == Ordering::Greater {
                hash = digest([&p[..], &hash[..]].concat()).as_bytes().to_owned();
            } else {
                hash = digest([&hash[..], &p[..]].concat()).as_bytes().to_owned();
            }
        }
        return &hash == root;
    }
}

fn main() -> Result<()> {
    let mut tree = MerkleTree::new(&vec![
        vec![0, 1, 2],
        vec![0, 1, 2],
        vec![1, 2, 3],
        vec![1, 2, 3],
        vec![1, 2, 3],
        vec![2, 3, 4],
        vec![2, 3, 4],
        vec![2, 3, 4],
        vec![2, 3, 4],
        vec![3, 4, 5],
        vec![3, 4, 5],
        vec![3, 4, 5],
        vec![3, 4, 5],
        vec![3, 4, 5],
    ])?;
    loop {
        for i in 0..tree.leaf_num() {
            println!("merkle tree: ");
            for node in tree.get_tree() {
                print!("{:?} ", node.len());
            }
            println!("");
            for node in tree.get_tree() {
                println!("{:?}", node);
            }

            println!("merkle tree height: {:?}", tree.tree_height());

            println!("merkle leaf number: {:?}", tree.leaf_num());

            let proof = tree.get_proof(i)?;
            println!("merkle tree proof:");
            for p in &proof.proof {
                println!("{:?}", p);
            }

            let verfiy_result = proof.verify_proof(&tree.get_leaf()[i], &tree.get_root());

            println!("verify result: {:?}", verfiy_result);
        }

        let mut leafs = tree.get_leaf().to_owned();
        leafs.pop();

        if leafs.len() <= 1 {
            break;
        }

        tree = MerkleTree::new(&leafs)?;
    }

    return Ok(());
}
