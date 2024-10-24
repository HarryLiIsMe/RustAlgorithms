use anyhow::Result;

mod merkle_tree;
use merkle_tree::{MerkleProof, MerkleTree, Node};

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
            for nodes in tree.get_tree() {
                print!("{:?} ", nodes.len());
            }
            println!("");
            for nodes in tree.get_tree() {
                for node in nodes.iter() {
                    println!("{:?}", node);
                }
            }

            println!("merkle tree height: {:?}", tree.tree_height());

            println!("merkle leaf number: {:?}", tree.leaf_num());

            let proof: MerkleProof = tree.get_proof(i)?;
            println!("merkle tree proof:");
            for p in &proof {
                println!("{:?}", p);
            }

            let verfiy_result = proof.verify_proof(&tree.get_leaf()[i], &tree.get_root());

            println!("verify result: {:?}", verfiy_result);
        }

        let mut leafs: Vec<Node> = tree.get_leaf().to_owned();
        leafs.pop();

        if leafs.len() <= 1 {
            break;
        }

        tree = MerkleTree::new(&leafs)?;
    }

    return Ok(());
}
