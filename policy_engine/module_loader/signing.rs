use crate::registry::Module;

/// Simulates signing a module (placeholder for real crypto system).
pub fn sign_module(module: &mut Module) {
    let base = format!("{}:{}", module.name, module.version);

    // fake hash generation (replace with SHA-256 later)
    module.hash = format!("{:x}", simple_hash(&base));
    module.signed = true;
}

fn simple_hash(input: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325;
    for b in input.bytes() {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
