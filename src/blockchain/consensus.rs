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
    fn verify_sub_chain_even(&self, _chain: &[Header]) -> bool {
        todo!()
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