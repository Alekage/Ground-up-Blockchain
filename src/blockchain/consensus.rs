#![allow(dead_code, unused_imports)]

use crate::hash;

type Hash = u64;

pub const THRESHOLD: u64 = u64::max_value()/100;

const FORK_HEIGHT: u64 = 2;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Header {
    parent: Hash,
    height: u64,
    extrinsic: u64,
    state: u64, 
    consensus_digest: u64
}


impl Header {
    fn genesis() -> Self {
        let genesis = Header {
            parent: 0,
            height: 0,
            extrinsic: 0,
            state: 0,
            consensus_digest: 0
           };
    
           genesis
    }
    fn child(&self, extrinsic: u64) -> Self {
        let mut header = Header {
            parent: hash(self),
            height: self.height + 1,
            extrinsic,
            state: self.state + extrinsic,
            consensus_digest: 0
        };

        solve_pow(&mut header);

        header 
    }
    
    fn verify_sub_chain(&self, chain: &[Header]) -> bool {
        let mut last_header = self;
    
        for block_header in chain {
            if block_header.parent != hash(last_header)
                || block_header.height != last_header.height + 1
                || block_header.state != last_header.state + block_header.extrinsic
                || hash(block_header) > THRESHOLD
            {
                return false;
            }
    
            last_header = block_header;
        }
    
        true
    }
    fn verify_sub_chain_even(&self, chain: &[Header]) -> bool {
        let mut last_header = self;

        for block_header in chain {
            if block_header.parent != hash(last_header)
                || block_header.height != last_header.height + 1
                || block_header.height > 2 && block_header.state % 2 != 0
                || block_header.state != last_header.state + block_header.extrinsic 
                || hash(block_header) > THRESHOLD
            {
                return false;
            }

            last_header = block_header;
        }
    
        true
    }


    fn verify_sub_chain_odd(&self, _chain: &[Header]) -> bool {
        todo!()
    }
}

// PoW helper function
fn solve_pow(header: &mut Header) -> u64 {
    while hash(header) >= THRESHOLD {
        header.consensus_digest += 1;
    };
    header.consensus_digest
}

fn build_contentious_forked_chain() -> (Vec<Header>, Vec<Header>, Vec<Header>) {
    todo!()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn bc_3_genesis_block_height() {
        let g = Header::genesis();
        assert!(g.height == 0);
    }

    #[test]
    fn bc_3_genesis_block_parent() {
        let g = Header::genesis();
        assert!(g.parent == 0);
    }

    #[test]
    fn bc_3_genesis_block_extrinsic() {
        // Typically genesis blocks do not have any extrinsics.
        // In Substrate they never do. So our convention is to have the extrinsic be 0.
        let g = Header::genesis();
        assert!(g.extrinsic == 0);
    }

    #[test]
    fn bc_3_genesis_block_state() {
        let g = Header::genesis();
        assert!(g.state == 0);
    }

    #[test]
    fn bc_3_genesis_consensus_digest() {
        // We could require that the genesis block have a valid proof of work as well.
        // But instead I've chosen the simpler path of defining the nonce = 0 in genesis.
        let g = Header::genesis();
        assert!(g.consensus_digest == 0);
    }

    #[test]
    fn bc_3_child_block_height() {
        let g = Header::genesis();
        let b1 = g.child(0);
        assert!(b1.height == 1);
    }

    #[test]
    fn bc_3_child_block_parent() {
        let g = Header::genesis();
        let b1 = g.child(0);
        assert!(b1.parent == hash(&g));
    }

    #[test]
    fn bc_3_child_block_extrinsic() {
        let g = Header::genesis();
        let b1 = g.child(7);
        assert_eq!(b1.extrinsic, 7);
    }

    #[test]
    fn bc_3_child_block_state() {
        let g = Header::genesis();
        let b1 = g.child(7);
        assert_eq!(b1.state, 7);
    }

    #[test]
    fn bc_3_child_block_consensus_digest() {
        let g = Header::genesis();
        let b1 = g.child(7);
        assert!(hash(&b1) < THRESHOLD);
    }

    #[test]
    fn bc_3_verify_genesis_only() {
        let g = Header::genesis();

        assert!(g.verify_sub_chain(&[]));
    }

    #[test]
    fn bc_3_verify_three_blocks() {
        let g = Header::genesis();
        let b1 = g.child(5);
        let b2 = b1.child(6);

        assert_eq!(b2.state, 11);
        assert!(g.verify_sub_chain(&[b1, b2]));
    }

    #[test]
    fn bc_3_cant_verify_invalid_parent() {
        let g = Header::genesis();
        let mut b1 = g.child(5);
        b1.parent = 10;

        assert!(!g.verify_sub_chain(&[b1]));
    }

    #[test]
    fn bc_3_cant_verify_invalid_number() {
        let g = Header::genesis();
        let mut b1 = g.child(5);
        b1.height = 10;

        assert!(!g.verify_sub_chain(&[b1]));
    }

    #[test]
    fn bc_3_cant_verify_invalid_state() {
        let g = Header::genesis();
        let mut b1 = g.child(5);
        b1.state = 10;

        assert!(!g.verify_sub_chain(&[b1]));
    }

    #[test]
    fn bc_3_cant_verify_invalid_pow() {
        let g = Header::genesis();
        let mut b1 = g.child(5);
        // It is possible that this test will pass with a false positive because
        // the PoW difficulty is relatively low.
        b1.consensus_digest = 10;

        assert!(!g.verify_sub_chain(&[b1]));
    }

    #[test]
    fn bc_3_even_chain_valid() {
        let g = Header::genesis(); // 0
        let b1 = g.child(2); // 2
        let b2 = b1.child(1); // 3
                            // It' all about the states, not the extrinsics. So once the state is even
                            // we need to keep it that way. So add evens
        let b3 = b2.child(1); // 4
        let b4 = b3.child(2); // 6

        assert!(g.verify_sub_chain_even(&[b1, b2, b3, b4]));
    }

    #[test]
    fn bc_3_even_chain_invalid_first_block_after_fork() {
        let g = Header::genesis(); // 0
        let b1 = g.child(2); // 2
        let b2 = b1.child(1); // 3
        let b3 = b2.child(2); // 5 - invalid
        let b4 = b3.child(1); // 6

        assert!(!g.verify_sub_chain_even(&[b1, b2, b3, b4]));
    }

    #[test]
    fn bc_3_even_chain_invalid_second_block_after_fork() {
        let g = Header::genesis(); // 0
        let b1 = g.child(2); // 2
        let b2 = b1.child(1); // 3
        let b3 = b2.child(1); // 4
        let b4 = b3.child(1); // 5 - invalid

        assert!(!g.verify_sub_chain_even(&[b1, b2, b3, b4]));
    }

    #[test]
    fn bc_3_odd_chain_valid() {
        let g = Header::genesis(); // 0
        let b1 = g.child(2); // 2
        let b2 = b1.child(1); // 3
                            // It' all about the states, not the extrinsics. So once the state is odd
                            // we need to keep it that way. So add evens
        let b3 = b2.child(2); // 5
        let b4 = b3.child(2); // 7

        assert!(g.verify_sub_chain_odd(&[b1, b2, b3, b4]));
    }

    #[test]
    fn bc_3_odd_chain_invalid_first_block_after_fork() {
        let g = Header::genesis(); // 0
        let b1 = g.child(2); // 2
        let b2 = b1.child(1); // 3
        let b3 = b2.child(1); // 4 - invalid
        let b4 = b3.child(1); // 5

        assert!(!g.verify_sub_chain_odd(&[b1, b2, b3, b4]));
    }

    #[test]
    fn bc_3_odd_chain_invalid_second_block_after_fork() {
        let g = Header::genesis(); // 0
        let b1 = g.child(2); // 2
        let b2 = b1.child(1); // 3
        let b3 = b2.child(2); // 5
        let b4 = b3.child(1); // 6 - invalid

        assert!(!g.verify_sub_chain_odd(&[b1, b2, b3, b4]));
    }

    #[test]
    fn bc_3_verify_forked_chain() {
        let (prefix, even, odd) = build_contentious_forked_chain();

        let g = &prefix[0];
        let full_even_chain = [&prefix[1..], &even].concat();
        let full_odd_chain = [&prefix[1..], &odd].concat();

        // Both chains are individually valid according to the original rules.
        assert!(g.verify_sub_chain(&full_even_chain[..]));
        assert!(g.verify_sub_chain(&full_odd_chain[..]));

        // Only the even chain is valid according to the even rules
        assert!(g.verify_sub_chain_even(&full_even_chain[..]));
        assert!(!g.verify_sub_chain_even(&full_odd_chain[..]));

        // Only the odd chain is valid according to the odd rules
        assert!(!g.verify_sub_chain_odd(&full_even_chain[..]));
        assert!(g.verify_sub_chain_odd(&full_odd_chain[..]));
    }

}