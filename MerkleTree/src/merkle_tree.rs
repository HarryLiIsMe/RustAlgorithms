use anyhow::{bail, Result};
use sha256::digest;
use std::{cmp::Ordering, mem::take, vec::Vec};

pub type Node = Vec<u8>;
pub type Hash = Vec<u8>;

pub struct MerkleTree {
    tree: Vec<Vec<Node>>,
}

impl MerkleTree {
    pub fn new(data: &Vec<Node>) -> Result<Self> {
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

    pub fn get_root(&self) -> &Node {
        return self.tree.last().unwrap().first().unwrap();
    }

    pub fn get_tree(&self) -> &Vec<Vec<Node>> {
        return &self.tree;
    }

    pub fn get_leaf(&self) -> &Vec<Node> {
        return self.tree.first().unwrap();
    }

    pub fn tree_height(&self) -> usize {
        return self.tree.len();
    }

    pub fn leaf_num(&self) -> usize {
        return self.tree.first().unwrap().len();
    }

    pub fn get_proof(&self, index: usize) -> Result<MerkleProof> {
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

pub struct MerkleProof {
    proof: Vec<Hash>,
}

impl MerkleProof {
    pub fn new(proof: Vec<Hash>) -> Result<Self> {
        if proof.is_empty() {
            bail!("merkle proof is empty!!!");
        }
        return Ok(MerkleProof { proof });
    }

    pub fn verify_proof(&self, leaf: &Node, root: &Node) -> bool {
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

impl IntoIterator for MerkleProof {
    type Item = Hash;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        return self.proof.into_iter();
    }
}

impl<'a> IntoIterator for &'a MerkleProof {
    type Item = &'a Hash;
    type IntoIter = std::slice::Iter<'a, Hash>;
    fn into_iter(self) -> Self::IntoIter {
        return self.proof.iter();
    }
}

impl<'a> IntoIterator for &'a mut MerkleProof {
    type Item = &'a mut Hash;
    type IntoIter = std::slice::IterMut<'a, Hash>;
    fn into_iter(self) -> Self::IntoIter {
        return self.proof.iter_mut();
    }
}
