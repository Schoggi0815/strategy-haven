use spacetimedb::{rand::Rng, StdbRng};

/// List of English adjective words
pub const ADJECTIVES: &[&str] = &include!(concat!(env!("OUT_DIR"), "/adjectives.rs"));

/// List of English noun words
pub const NOUNS: &[&str] = &include!(concat!(env!("OUT_DIR"), "/nouns.rs"));

pub fn generate_username(rng: &StdbRng) -> String {
    generate_random_name(rng)
}

fn generate_random_name(mut rng: &StdbRng) -> String {
    let adjective = ADJECTIVES
        .get(rng.gen_range(0..ADJECTIVES.len()))
        .expect("FAIL");
    let noun = NOUNS.get(rng.gen_range(0..NOUNS.len())).expect("FAIL");
    format!("{adjective}-{noun}")
}
