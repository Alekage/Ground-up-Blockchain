#![allow(dead_code)]

//use crate::hash;


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