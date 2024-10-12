#![allow(dead_code, unused_imports)]

use crate::hash;
use super::consensus::THRESHOLD;
type Hash = u64;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Header {
    parent: Hash,
    height: u64,
    extrinsics_root: Hash,
    state: u64,
    pub consensus_digest: u64,
}

impl Header {
    pub fn genesis() -> Self {
        todo!()
    }

    pub fn child(&self, _extrinsics_root: Hash, _state: u64) -> Self {
        todo!()
    }

    fn verify_child(&self, _child: &Header) -> bool {
        todo!()
    }

    fn verify_sub_chain(&self, _chain: &[Header]) -> bool {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Block {
    pub(crate) header: Header,
    pub(crate) body: Vec<u64>,
}

impl Block {
    pub fn genesis() -> Self {
        todo!("Exercise 5")
    }

    pub fn child(&self, _extrinsics: Vec<u64>) -> Self {
        todo!("Exercise 6")
    }

    pub fn verify_sub_chain(&self, _chain: &[Block]) -> bool {
        todo!("Exercise 7")
    }
}

fn build_invalid_child_block_with_valid_header(_parent: &Header) -> Block {
    todo!("Exercise 8")
}