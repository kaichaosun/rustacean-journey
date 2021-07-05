use ring::digest::{digest, Context, SHA256};

#[derive(Debug)]
pub struct MerkleTree {
    root: Tree,
    height: usize,
    count: usize,
}

#[derive(Debug)]
pub enum Tree {
    Empty {
        hash: Vec<u8>,
    },

    Leaf {
        hash: Vec<u8>,
        value: u8,
    },

    Node {
        hash: Vec<u8>,
        left: Box<Tree>,
        right: Box<Tree>,
    },
}

impl Tree {
    pub fn empty() -> Self {
        Tree::Empty {
            hash: digest(&SHA256, &[]).as_ref().into(),
        }
    }

    pub fn new_leaf(value: u8) -> Tree {
        let mut ctx = Context::new(&SHA256);
        ctx.update(&[0x00]);
        ctx.update(&[value]);
        let hash = ctx.finish();
        Tree::Leaf {
            hash: hash.as_ref().into(),
            value,
        }
    }

    pub fn hash(&self) -> &Vec<u8> {
        match *self {
            Tree::Empty { ref hash } => hash,
            Tree::Leaf { ref hash, .. } => hash,
            Tree::Node { ref hash, .. } => hash,
        }
    }

    pub fn new_node(left: Tree, right: Tree) -> Tree {
        let mut ctx = Context::new(&SHA256);
        ctx.update(&[0x01]);
        ctx.update(left.hash());
        ctx.update(right.hash());
        let hash = ctx.finish();

        Tree::Node {
            hash: hash.as_ref().into(),
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

impl MerkleTree {
    pub fn from_vec(values: Vec<u8>) -> Self {
        if values.is_empty() {
            return MerkleTree {
                root: Tree::empty(),
                height: 0,
                count: 0,
            };
        }

        let count = values.len();
        let mut height = 0;
        let mut cur = Vec::with_capacity(count);

        for v in values {
            let leaf = Tree::new_leaf(v);
            cur.push(leaf);
        }

        while cur.len() > 1 {
            let mut next = Vec::new();
            while !cur.is_empty() {
                if cur.len() == 1 {
                    next.push(cur.remove(0));
                } else {
                    let left = cur.remove(0);
                    let right = cur.remove(0);

                    let node = Tree::new_node(left, right);
                    next.push(node);
                }
            }
            height += 1;
            cur = next;
        }

        debug_assert!(cur.len() == 1);

        let root = cur.remove(0);

        MerkleTree {
            root: root,
            height,
            count,
        }
    }

    // TODO: Generate merkle proof
    // pub fn gen_proof(&self, value: u8) -> Option<Proof> {
    //     let root_hash = self.root.hash();
        
    //     let mut ctx = Context::new(&SHA256);
    //     ctx.update(&[0x00]);
    //     ctx.update(&[value]);
    //     let leaf_hash = ctx.finish();
    // }

}

fn main() {
    println!("Merkle Tree journey starts!");

    let leafs = vec![2, 8, 9, 18, 4];
    let merkle_tree = MerkleTree::from_vec(leafs);

    println!("The constructed merkle tree is:\n{:?}", merkle_tree);
}
