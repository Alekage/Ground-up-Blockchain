use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

mod state_machine;
mod blockchain;

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
