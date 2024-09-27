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