use rand::rngs::ThreadRng;
use rand::Rng;

pub fn rand_range(rng: &mut ThreadRng, a: i16, b: i16) -> i16 {
    rng.gen_range(a..b)
}

pub fn rand_fraction(rng: &mut ThreadRng) -> f32 {
    rand_range(rng, 0, 10000) as f32 / 10000.0
}

pub fn bernoulli(rng: &mut ThreadRng, p: f32) -> bool {
    rand_range(rng, 0, 1000) < ((1000. * p) as i16)
}

pub fn coin_flip(rng: &mut ThreadRng) -> bool {
    rand_range(rng, 0, 2) > 0
}
