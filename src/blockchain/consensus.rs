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
        todo!()
    }
    fn child(&self, _extrinsic: u64) -> Self {
        todo!()
    }
    fn verify_sub_chain(&self, _chain: &[Header]) -> bool {
        todo!()
    }
    fn verify_sub_chain_even(&self, _chain: &[Header]) -> bool {
        todo!()
    }
    fn verify_sub_chain_odd(&self, _chain: &[Header]) -> bool {
        todo!()
    }
}

fn build_contentious_forked_chain() -> (Vec<Header>, Vec<Header>, Vec<Header>) {
    todo!()
}