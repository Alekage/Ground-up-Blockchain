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
        Header { parent: hash(&self), height: self.height + 1 , extrinsic , state: self.state + extrinsic, consensus_digest: () }
    }

    fn verify_sub_chain(&self, chain: &[Header]) -> bool {
        let mut last_header = self; 

        for block_header in chain {
            if block_header.parent != hash(last_header) || block_header.height != last_header.height + 1 
            || block_header.state != last_header.state + block_header.extrinsic {
                return false
            }

            last_header = block_header;
        }

        true    
    }

}


fn build_valid_chain(n: (u64, Vec<u64>)) -> Vec<Header> {
    let genesis = Header {
        parent: 0,
        height: 0,
        extrinsic: 0,
        state: 0,
        consensus_digest: (),
    };

    let (_, extrinsics) = n; 

    extrinsics.into_iter()
        .scan(genesis, |header_now, extrinsic| {
            let current = header_now.clone();
            *header_now = current.child(extrinsic); 
            Some(current)
        })
        .collect()
}


fn build_forked_chain() -> (Vec<Header>, Vec<Header>) {
    let genesis = Header {
        parent: 0,
        height: 0,
        extrinsic: 0,
        state: 0,
        consensus_digest: (),
    };

    let block_1 = genesis.child(1);
    let block_2 = block_1.child(3);

    let block_1_prim = genesis.child(2);
    let block_2_prim = block_1_prim.child(3);

    (vec![genesis.clone(), block_1, block_2], vec![genesis, block_1_prim, block_2_prim])
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

    #[test]
    fn bc_2_child_block_extrinsic() {
        let g = Header::genesis();
        let b1 = g.child(7);
        assert_eq!(b1.extrinsic, 7);
    }

    #[test]
    fn bc_2_child_block_state() {
        let g = Header::genesis();
        let b1 = g.child(7);
        assert_eq!(b1.state, 7);
    }

    #[test]
    fn bc_2_verify_genesis_only() {
        let g = Header::genesis();

        assert!(g.verify_sub_chain(&[]));
    }

    #[test]
    fn bc_2_verify_three_blocks() {
        let g = Header::genesis();
        let b1 = g.child(5);
        let b2 = b1.child(6);

        assert_eq!(b2.state, 11);
        assert!(g.verify_sub_chain(&[b1, b2]));
    }

    #[test]
    fn bc_2_cant_verify_invalid_parent() {
        let g = Header::genesis();
        let mut b1 = g.child(5);
        b1.parent = 10;

        assert!(!g.verify_sub_chain(&[b1]));
    }

    #[test]
    fn bc_2_cant_verify_invalid_number() {
        let g = Header::genesis();
        let mut b1 = g.child(5);
        b1.height = 10;

        assert!(!g.verify_sub_chain(&[b1]));
    }

    #[test]
    fn bc_2_cant_verify_invalid_state() {
        let g = Header::genesis();
        let mut b1 = g.child(5);
        b1.state = 10;

        assert!(!g.verify_sub_chain(&[b1]));
    }

    #[test]
    fn bc_2_verify_forked_chain() {
        let g = Header::genesis();
        let (c1, c2) = build_forked_chain();

        assert_eq!(g, c1[0]);
        assert_eq!(g, c2[0]);

        assert!(g.verify_sub_chain(&c1[1..]));
        assert!(g.verify_sub_chain(&c2[1..]));

        assert_ne!(c1.last(), c2.last());
    }

    #[test]
    fn bc_2_verify_valid_chain() {
        let genesis = Header::genesis();
        let blockchain = (5, vec![0, 2, 7, 9, 6]);
        let built_blockchain = build_valid_chain(blockchain);

        assert!(genesis.verify_sub_chain(&built_blockchain[1..]))
    }

}