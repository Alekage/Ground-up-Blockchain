#![allow(dead_code)]

use crate::hash;

type Hash = u64;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Header {
    parent: Hash,
    height: u64,
    extrinsics_root: (),
    state_root: (),
    consensus_digest: (),
}

impl Header {
    fn genesis() -> Self {
        Header { parent: 0, height: 0, extrinsics_root: (), state_root: (), consensus_digest: () }
    }

    fn child(&self) -> Self {
        Header { parent: hash(&self), height: self.height + 1 , extrinsics_root:() , state_root: (), consensus_digest: () }
    }

    fn verify_sub_chain(&self, chain: &[Header]) -> bool {
        let mut last_header = self; 

        for block_header in chain {
            if block_header.parent != hash(last_header) || block_header.height != last_header.height + 1 {
                return false
            }

            last_header = block_header;
        }

        true        
    }
}

fn build_valid_chain_length_5() -> Vec<Header> {
    todo!()
}

fn build_an_invalid_chain() -> Vec<Header> {
    todo!()
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn bc_1_genesis_block_height() {
        let g = Header::genesis();
        assert!(g.height == 0);
    }

    #[test]
    fn bc_1_genesis_block_parent() {
        let g = Header::genesis();
        assert!(g.parent == 0);
    }

    #[test]
    fn bc_1_child_block_height() {
        let g = Header::genesis();
        let b1 = g.child();
        assert!(b1.height == 1);
    }

    #[test]
    fn bc_1_child_block_parent() {
        let g = Header::genesis();
        let b1 = g.child();
        assert!(b1.parent == hash(&g));
    }

    #[test]
    fn bc_1_verify_genesis_only() {
        let g = Header::genesis();

        assert!(g.verify_sub_chain(&[]));
    }

    #[test]
    fn bc_1_verify_three_blocks() {
        let g = Header::genesis();
        let b1 = g.child();
        let b2 = b1.child();

        assert!(g.verify_sub_chain(&[b1, b2]));
    }

    #[test]
    fn bc_1_cant_verify_invalid_height() {
        let g = Header::genesis();
        let mut b1 = g.child();
        b1.height = 10;

        assert!(!g.verify_sub_chain(&[b1]))
    }

    #[test]
    fn bc_1_cant_verify_invalid_parent() {
        let g = Header::genesis();
        let mut b1 = g.child();
        b1.parent = 10;

        assert!(!g.verify_sub_chain(&[b1]))
    }

    #[test]
    fn bc_1_verify_chain_length_five() {
        let chain = build_valid_chain_length_5();
        assert!(chain[0].verify_sub_chain(&chain[1..]))
    }

    #[test]
    fn bc_1_invalid_chain_is_really_invalid() {
        let invalid_chain = build_an_invalid_chain();
        assert!(!invalid_chain[0].verify_sub_chain(&invalid_chain[1..]))
    }

}