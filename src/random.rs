use rand::rngs::ThreadRng;
use rand::Rng;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub fn rand_range(rng: &mut ChaCha8Rng, a: i16, b: i16) -> i16 {
    rng.gen_range(a..b)
}

pub fn rand_fraction(rng: &mut ChaCha8Rng) -> f32 {
    rand_range(rng, 0, 10000) as f32 / 10000.0
}

pub fn bernoulli(rng: &mut ChaCha8Rng, p: f32) -> bool {
    rand_range(rng, 0, 1000) < ((1000. * p) as i16)
}

pub fn coin_flip(rng: &mut ThreadRng) -> bool {
    rand_range(rng, 0, 2) > 0
}
