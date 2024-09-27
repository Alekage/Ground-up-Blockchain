#![allow(dead_code, unused_imports)]

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
        todo!()
    }

    fn child(&self) -> Self {
        todo!()
    }

    fn verify_sub_chain(&self, _chain: &[Header]) -> bool {
        todo!()
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