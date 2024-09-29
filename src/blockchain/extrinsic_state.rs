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
        Header { parent: 0, height: 0, extrinsic: 0, state: 0, consensus_digest: () }
    }


    fn child(&self, extrinsic: u64) -> Self {
        Header { parent: hash(&self), height: self.height + 1 , extrinsic , state: self.extrinsic + extrinsic, consensus_digest: () }
    }

    fn verify_sub_chain(&self, chain: &[Header]) -> bool {
        let mut last_header = self; 

        for block_header in chain {
            if block_header.parent != hash(last_header) || block_header.height != last_header.height + 1 || block_header.state != last_header.state + self.extrinsic {
                return false
            }

            last_header = block_header;
        }

        true    
    }

}


fn build_valid_chain(n: u64) -> Vec<Header> {
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