#![allow(dead_code, unused_imports, unused_variables)]

use crate::hash;

type Hash = u64;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Header {
    parent: Hash,
    height: u64,
    extrinsic: u64,
    state: u64,
    consensus_digest: (),
}

impl Header {

    fn genesis() -> Self {
        todo!()
    }


    fn child(&self, extrinsic: u64) -> Self {
        todo!()
    }


    fn verify_sub_chain(&self, chain: &[Header]) -> bool {
        todo!()
    }
}


fn build_valid_chain(n: u64) -> Vec<Header> {
    todo!()
}

fn build_an_invalid_chain() -> Vec<Header> {
    todo!()
}


fn build_forked_chain() -> (Vec<Header>, Vec<Header>) {
    todo!()
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn bc_2_genesis_block_height() {
        let g = Header::genesis();
        assert!(g.height == 0);
    }

    #[test]
    fn bc_2_genesis_block_parent() {
        let g = Header::genesis();
        assert!(g.parent == 0);
    }

    #[test]
    fn bc_2_genesis_block_extrinsic() {
        // Typically genesis blocks do not have any extrinsics.
        // In Substrate they never do. So our convention is to have the extrinsic be 0.
        let g = Header::genesis();
        assert!(g.extrinsic == 0);
    }

    #[test]
    fn bc_2_genesis_block_state() {
        let g = Header::genesis();
        assert!(g.state == 0);
    }

    #[test]
    fn bc_2_child_block_height() {
        let g = Header::genesis();
        let b1 = g.child(0);
        assert!(b1.height == 1);
    }

    #[test]
    fn bc_2_child_block_parent() {
        let g = Header::genesis();
        let b1 = g.child(0);
        assert!(b1.parent == hash(&g));
    }
}